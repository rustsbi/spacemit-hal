use super::Board;
use crate::image::{self, LoadedImages};
use core::ptr;
use embedded_hal::delay::DelayNs;
use qspi_nor::{NorFlash, ReadNorFlash};
use spacemit_hal::{clock::Hertz, counter::CounterDelay, qspi};
use spacemit_rt::soc::k1::{self, MFPR};

pub(super) struct Nor {
    pub qspi: &'static mut k1::QSPI,
    pub apmu: &'static mut k1::APMU,
    pub pads: (
        &'static mut k1::Pad<98>,
        &'static mut k1::Pad<99>,
        &'static mut k1::Pad<100>,
        &'static mut k1::Pad<101>,
        &'static mut k1::Pad<102>,
        &'static mut k1::Pad<103>,
    ),
    pub delay: CounterDelay<'static>,
}

impl Board {
    /// Reads and verifies the official NOR images into trained DRAM without executing them.
    ///
    /// # Safety
    /// Call after successful DDR training on the sole M-mode boot hart.
    ///
    /// Reserve DRAM 0..128 MiB; stop all DMA, QSPI IRQ, AHB/XIP and other firmware users.
    ///
    /// The application stack and its linker-reserved guard must remain exclusive.
    pub unsafe fn load_images(
        &mut self,
        product_name: Option<&str>,
    ) -> Result<LoadedImages, image::Error> {
        let bottom = ptr::addr_of!(__stack_guard) as usize;
        // SAFETY: The guard lies below the live application stack; DRAM is trained and unused.
        unsafe {
            ptr::write_bytes(bottom as *mut u8, 0xa5, 256);
            let result = self.read_images(product_name);
            image::check_stack_guard(bottom)?;
            result
        }
    }

    unsafe fn read_images(
        &mut self,
        product_name: Option<&str>,
    ) -> Result<LoadedImages, image::Error> {
        let frequency = self
            .clocks
            .qspi_source()
            .ok_or(image::Error::Qspi(qspi::Error::ClockDisabled))?;
        // SAFETY: The caller excludes all flash users and Board retains the source clocks.
        let mut backend = unsafe { self.nor.initialize(frequency)? };
        let mut flash = NorFlash::probe(&mut backend, qspi_nor::Config::default())?;
        let capacity = flash.capacity();
        let id = flash.jedec_id()?;
        crate::println!(
            "NOR: JEDEC {} {} {}, {} bytes",
            id[0],
            id[1],
            id[2],
            capacity
        );
        // SAFETY: The caller reserves trained DRAM for staging and validated payload writes.
        unsafe { image::load(&mut flash, product_name) }
    }
}

unsafe extern "C" {
    static __stack_guard: u8;
}

impl Nor {
    unsafe fn initialize(&mut self, parent: Hertz) -> Result<qspi::BlockingQspi<'_>, qspi::Error> {
        let clock = &self.apmu.qspi_clock_reset;
        let current = clock.read();
        if current.is_frequency_change_pending() {
            return Err(qspi::Error::ClockDisabled);
        }
        // Vendor SPL ccu-k1x.c: selector 5 = PLL1 / 23; divide by 5 stays below 26.5 MHz.
        let reset = current
            .with_bus_reset_asserted(true)
            .with_reset_asserted(true)
            .with_bus_clock_enabled(false)
            .with_functional_clock_enabled(false)
            .with_clock_source_selector(5)
            .with_clock_divider(5);
        let gated = reset
            .with_bus_clock_enabled(true)
            .with_functional_clock_enabled(true);
        let running = gated
            .with_bus_reset_asserted(false)
            .with_reset_asserted(false);
        // SAFETY: Exclusive APMU token; no active QSPI/AHB clients and the PLL source is stable.
        unsafe {
            clock.write(reset);
            riscv::asm::fence();
            self.delay.delay_us(10);
            clock.write(gated);
            riscv::asm::fence();
            self.delay.delay_us(10);
            clock.write(running);
            riscv::asm::fence();
            self.delay.delay_us(10);
        }
        if clock.read() != running {
            return Err(qspi::Error::ClockDisabled);
        }
        // k1-x_pinctrl.dtsi pinctrl_qspi: AF0, edge disabled, 3.3 V DS4, CS pulled up.
        let _pads = &mut self.pads;
        // SAFETY: These six pad tokens retain exclusive access to exactly GPIO98..103.
        unsafe {
            let pads = &(*MFPR::ptr()).gpio;
            for pin in 98..=103 {
                pads[pin].write(if pin == 103 { 0xc440 } else { 0x0440 });
                riscv::asm::fence();
            }
            qspi::BlockingQspi::from_bootrom(
                &mut *self.qspi,
                clock,
                qspi::Config::new(Hertz(parent.0 / 5)),
                &mut self.delay,
            )
        }
    }
}
