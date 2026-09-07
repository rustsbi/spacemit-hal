//! MUSE Card M1 board initialization.

use crate::{Error, Result, console};
use eeprom24x::{Eeprom24x, SlaveAddr, addr_size, page_size, unique_serial};
use measurements::Voltage;
use spacemit_hal::{
    clock::{Clocks, Hertz, I2cClock, Strict, UartClock},
    counter::CounterDelay,
    i2c::{BlockingI2c, Config},
    prelude::*,
    uart,
};
use spacemit_p1::{Buck, Spm8821};
use spacemit_rt::soc::k1::{self, Peripherals};

#[cfg(feature = "ddr")]
mod nor;

const CLOCK_CONFIG: Strict = {
    let config = Strict::new()
        .reference_clock(Hertz(24_000_000))
        .i2c_clock(true);
    if cfg!(feature = "ddr") {
        config.qspi_clock(true)
    } else {
        config
    }
};

// Vendor k1-x_MUSE-Card.dts, k1-x_spm8821.dtsi and include/power/spacemit/spm8821.h.
// BUCK1: 500 mV + 0x6e * 5 mV = 1.05 V; BUCK3 retains its existing voltage.

/// Supported board devices backed by the entry's permanent peripheral storage.
pub struct Board {
    /// Board parameter EEPROM.
    pub eeprom:
        Eeprom24x<BlockingI2c<'static>, page_size::B8, addr_size::OneByte, unique_serial::No>,
    /// Board power controller.
    pub pmic: Spm8821<BlockingI2c<'static>>,
    /// Validated board clocks.
    pub clocks: Clocks<'static>,
    #[cfg(feature = "ddr")]
    nor: nor::Nor,
}

#[cfg(feature = "ddr")]
impl Board {
    /// Trains LPDDR4X, verifies DRAM and returns the raw firmware value.
    ///
    /// # Safety
    /// Call once on the cold-boot M-mode hart, with all other harts and DMA stopped.
    ///
    /// DRAM must contain no live data; keep link.x's SRAM reservations exclusive.
    ///
    /// The board must use LPDDR4X, two chip selects and 80-ohm TX ODT.
    pub unsafe fn init_ddr(
        &mut self,
        recorded: Option<&crate::eeprom::DdrConfig<'_>>,
    ) -> core::result::Result<i32, crate::ddr::Error> {
        // Vendor arch/riscv/dts/k1-x_spl.dts; these are not EEPROM parser defaults.
        const CONFIG: crate::ddr::Config = crate::ddr::Config {
            chip_selects: 2,
            data_rate: 2400,
        };
        if let Some(info) = recorded
            && (info
                .chip_selects
                .is_some_and(|value| u32::from(value) != CONFIG.chip_selects)
                || info
                    .data_rate
                    .is_some_and(|value| u32::from(value) != CONFIG.data_rate)
                || info
                    .memory_type
                    .is_some_and(|value| !value.eq_ignore_ascii_case("LPDDR4X"))
                || info.tx_odt.is_some_and(|value| value != 80))
        {
            return Err(crate::ddr::Error::Config);
        }
        // SAFETY: The caller reserves SRAM/DRAM and the board's clocks and rails are ready.
        unsafe { crate::ddr::init(&CONFIG) }
    }
}

/// Internal board initialization for the entry macro.
#[doc(hidden)]
pub fn init_board(p: &'static mut Peripherals) -> Result<Board> {
    let clocks = CLOCK_CONFIG.configure(&mut p.apbs, &mut p.mpmu, &mut p.apbc_clocks.counter)?;
    init_console(
        &mut p.uart0,
        (&mut p.gpio.gpio68, &mut p.gpio.gpio69),
        &mut p.apbc_clocks.uart0,
        &clocks,
    )?;
    let mut delay = CounterDelay::new(&mut p.counter, &clocks)?;
    let eeprom = init_eeprom(
        &mut p.i2c2,
        (&mut p.gpio.gpio84, &mut p.gpio.gpio85),
        &mut p.apbc_clocks.i2c2,
        &clocks,
        &mut delay,
    )?;
    let mut pmic = init_pmic(
        &mut p.i2c8,
        &mut p.pmic_pads,
        &mut p.apbc_clocks.i2c8,
        &clocks,
        &mut delay,
    )?;
    configure_pmic(&mut pmic)?;
    delay.delay_ms(10);
    Ok(Board {
        eeprom,
        pmic,
        clocks,
        #[cfg(feature = "ddr")]
        nor: nor::Nor {
            qspi: &mut p.qspi,
            apmu: &mut p.apmu,
            pads: (
                &mut p.gpio.gpio98,
                &mut p.gpio.gpio99,
                &mut p.gpio.gpio100,
                &mut p.gpio.gpio101,
                &mut p.gpio.gpio102,
                &mut p.gpio.gpio103,
            ),
            delay,
        },
    })
}

// Keep each device's temporary setup state out of the shared init_board frame.
#[inline(never)]
fn init_console(
    uart: &'static mut k1::UART0,
    pads: (&'static mut k1::Pad<68>, &'static mut k1::Pad<69>),
    clock: &'static mut UartClock<k1::UART0>,
    clocks: &Clocks<'static>,
) -> Result<()> {
    let uart = uart.blocking(pads, clock.with_clock(clocks)?, uart::Config::default())?;
    console::install(uart).map_err(|_| Error::ConsoleUnavailable)
}

#[inline(never)]
fn init_eeprom(
    i2c: &'static mut k1::I2C2,
    pads: (&'static mut k1::Pad<84>, &'static mut k1::Pad<85>),
    clock: &'static mut I2cClock<k1::I2C2>,
    clocks: &Clocks<'static>,
    delay: &mut CounterDelay<'static>,
) -> Result<Eeprom24x<BlockingI2c<'static>, page_size::B8, addr_size::OneByte, unique_serial::No>> {
    let bus = i2c.blocking(pads, clock.with_clock(clocks)?, Config::default(), delay)?;
    Ok(Eeprom24x::new_24x02(bus, SlaveAddr::default()))
}

#[inline(never)]
fn init_pmic(
    i2c: &'static mut k1::I2C8,
    pads: &'static mut k1::PmicPads,
    clock: &'static mut I2cClock<k1::I2C8>,
    clocks: &Clocks<'static>,
    delay: &mut CounterDelay<'static>,
) -> Result<Spm8821<BlockingI2c<'static>>> {
    let bus = i2c.blocking(pads, clock.with_clock(clocks)?, Config::default(), delay)?;
    Ok(Spm8821::new(bus))
}

#[inline(never)]
fn configure_pmic(pmic: &mut Spm8821<BlockingI2c<'static>>) -> Result<()> {
    pmic.set_buck_voltage(Buck::Buck1, Voltage::from_millivolts(1050.0))?;
    pmic.enable_buck(Buck::Buck1)?;
    pmic.enable_buck(Buck::Buck3)?;
    Ok(())
}
