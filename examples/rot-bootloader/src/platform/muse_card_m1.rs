//! MUSE Card M1 board initialization.

use crate::{Error, Result, console};
use eeprom24x::{Eeprom24x, SlaveAddr, addr_size, page_size, unique_serial};
use measurements::Voltage;
use spacemit_hal::{
    clock::{Clocks, Hertz, Strict},
    counter::CounterDelay,
    i2c::{BlockingI2c, Config},
    prelude::*,
    uart,
};
use spacemit_p1::{Buck, Spm8821};
use spacemit_rt::soc::k1::Peripherals;

const CLOCK_CONFIG: Strict = Strict::new()
    .reference_clock(Hertz(24_000_000))
    .i2c_clock(true);

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
}

/// Internal board initialization for the entry macro.
#[doc(hidden)]
pub fn init_board(p: &'static mut Peripherals) -> Result<Board> {
    let clocks = CLOCK_CONFIG.configure(&mut p.apbs, &mut p.mpmu, &mut p.apbc_clocks.counter)?;
    let uart = (&mut p.uart0).blocking(
        (&mut p.gpio.gpio68, &mut p.gpio.gpio69),
        p.apbc_clocks.uart0.with_clock(&clocks)?,
        uart::Config::default(),
    )?;
    console::install(uart).map_err(|_| Error::ConsoleUnavailable)?;
    let mut delay = CounterDelay::new(&mut p.counter, &clocks)?;
    let eeprom_bus = (&mut p.i2c2).blocking(
        (&mut p.gpio.gpio84, &mut p.gpio.gpio85),
        p.apbc_clocks.i2c2.with_clock(&clocks)?,
        Config::default(),
        &mut delay,
    )?;
    let eeprom = Eeprom24x::new_24x02(eeprom_bus, SlaveAddr::default());
    let pmic_bus = (&mut p.i2c8).blocking(
        &mut p.pmic_pads,
        p.apbc_clocks.i2c8.with_clock(&clocks)?,
        Config::default(),
        &mut delay,
    )?;
    let mut pmic = Spm8821::new(pmic_bus);
    pmic.set_buck_voltage(Buck::Buck1, Voltage::from_millivolts(1050.0))?;
    pmic.enable_buck(Buck::Buck1)?;
    pmic.enable_buck(Buck::Buck3)?;
    delay.delay_ms(10);
    Ok(Board {
        eeprom,
        pmic,
        clocks,
    })
}
