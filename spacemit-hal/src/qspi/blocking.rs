use super::{Backend, Instance, RegisterBlock, Transfer, lut};
use crate::{
    apmu::QspiClockReset,
    clock::{Hertz, io_fence},
};
use embedded_hal::delay::DelayNs;
use volatile_register::RW;

const AMBA_BASE: u32 = 0xb800_0000;
const TRANSFER_FINISHED: u32 = 1;
const XIP: u32 = 1 << 1;
const IP_ERROR: u32 = 1 << 6;
const IP_FLAGS: u32 = TRANSFER_FINISHED | IP_ERROR;
const DISABLED: u32 = 1 << 14;
const SEQUENCE: u32 = 15;

/// Controller chip-select output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipSelect {
    /// Port A, chip 1.
    #[default]
    A1,
    /// Port A, chip 2.
    A2,
    /// Port B, chip 1.
    B1,
    /// Port B, chip 2.
    B2,
}

/// Polling configuration at an already established serial clock rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// Actual functional clock, retained without changing the clock tree.
    pub frequency: Hertz,
    /// Connected chip-select output.
    pub chip_select: ChipSelect,
    /// Maximum status reads per wait or register-write retry, not a duration.
    pub poll_budget: u32,
}

impl Config {
    /// Selects CS0 and a bounded poll budget at the supplied clock rate.
    pub const fn new(frequency: Hertz) -> Self {
        Self {
            frequency,
            chip_select: ChipSelect::A1,
            poll_budget: 100_000,
        }
    }
}

/// QSPI configuration or transfer failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Frequency or poll budget is outside the supported range.
    InvalidConfig,
    /// A clock gate is disabled, reset is asserted, or a frequency change is pending.
    ClockDisabled,
    /// The transfer cannot fit the address, LUT, or FIFO limits.
    InvalidTransfer,
    /// The controller is still in XIP mode.
    XipActive,
    /// A bounded hardware wait was exhausted.
    PollLimit,
    /// The controller rejected an IP command, preserving the flag snapshot.
    IpCommand(u32),
    /// A failed transfer has not reached idle; reconstruct after platform recovery.
    RecoveryRequired,
}

/// An exclusive, single-chip polling QSPI backend without AHB/XIP access.
pub struct BlockingQspi<'a> {
    registers: &'a RegisterBlock,
    _clock: &'a RW<QspiClockReset>,
    delay: &'a mut dyn DelayNs,
    config: Config,
    failed: bool,
}

impl<'a> BlockingQspi<'a> {
    /// Takes over BootROM's clocks and pads for bounded SDR IP transfers.
    ///
    /// # Safety
    /// Retain exclusive QSPI, clock and routed-pad access for 'a; stop all IRQ,
    /// DMA, AHB/XIP and other firmware users, including instruction fetches.
    ///
    /// Mappings and power must remain valid; `clock` must be the matching APMU
    /// register, with a stable bus clock of at least 1 MHz and the stated functional rate.
    ///
    /// Connected pads and flash must support the selected SDR rate and routing.
    pub unsafe fn from_bootrom(
        qspi: impl Instance<'a>,
        clock: &'a RW<QspiClockReset>,
        config: Config,
        delay: &'a mut dyn DelayNs,
    ) -> Result<Self, Error> {
        if config.poll_budget == 0 || !(1_000_000..=102_000_000).contains(&config.frequency.0) {
            return Err(Error::InvalidConfig);
        }
        let gate = clock.read();
        if !gate.is_bus_clock_enabled()
            || !gate.is_functional_clock_enabled()
            || gate.is_bus_reset_asserted()
            || gate.is_reset_asserted()
            || gate.is_frequency_change_pending()
        {
            return Err(Error::ClockDisabled);
        }
        let mut this = Self {
            registers: qspi.register_block(),
            _clock: clock,
            delay,
            config,
            failed: false,
        };
        this.idle()?;
        this.reset()?;
        let r = this.registers;
        // SAFETY: Exclusive idle controller, live clocks and no mapped-memory users.
        unsafe {
            r.module_control
                .write_configuration(r.module_control.read_configuration() | DISABLED);
            io_fence();
            r.interrupt_dma_enable.write(0);
            r.flash_control.write(r.flash_control.read() & !(3 << 16));
            r.sampling
                .write(r.sampling.read() & !((7 << 16) | (1 << 6) | (1 << 5) | 1));
            r.soc_control.write(8);
            r.flash_address_control.write(0);
            // IP addresses are emitted by LUT MODE instructions; SFAR only selects the chip.
            for (index, top) in r.flash_top_address.iter().enumerate() {
                top.write(AMBA_BASE + (index as u32 + 1) * 0x400);
            }
            io_fence();
            r.module_control.write_configuration(
                (r.module_control.read_configuration() & !(DISABLED | (1 << 7) | 3)) | 0x000f_000c,
            );
            io_fence();
        }
        this.retry_write(|| {
            // SAFETY: No transfer is active; select the IP-visible receive buffer.
            unsafe { r.receive_buffer_control.write(1 << 8) };
        })?;
        Ok(this)
    }

    fn idle(&self) -> Result<(), Error> {
        for _ in 0..self.config.poll_budget {
            if self.registers.flags.read() & XIP != 0 {
                return Err(Error::XipActive);
            }
            if self.registers.status.read() & 7 == 0 {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::PollLimit)
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.idle()?;
        let control = &self.registers.module_control;
        let value = control.read_configuration() & !3;
        // SAFETY: Both reset domains are owned and idle, with no XIP/AHB users.
        unsafe { control.write_configuration(value | 3) };
        io_fence();
        // At least one bus cycle plus two serial cycles; both clocks are >= 1 MHz.
        self.delay.delay_us(3);
        // SAFETY: The required reset pulse elapsed with valid clocks.
        unsafe { control.write_configuration(value) };
        io_fence();
        self.idle()
    }

    fn retry_write(&mut self, mut write: impl FnMut()) -> Result<(), Error> {
        for _ in 0..self.config.poll_budget {
            // SAFETY: Only acknowledge the documented retryable IP error.
            unsafe { self.registers.flags.clear(IP_ERROR) };
            io_fence();
            write();
            io_fence();
            if self.registers.flags.read() & IP_ERROR == 0 {
                return Ok(());
            }
            // Vendor workaround for rejected SFAR/RBCT writes.
            self.delay.delay_us(3_000);
        }
        Err(Error::IpCommand(self.registers.flags.read()))
    }

    fn prepare(&mut self, words: [u32; 4]) -> Result<(), Error> {
        if self.failed {
            return Err(Error::RecoveryRequired);
        }
        self.idle()?;
        let r = self.registers;
        // SAFETY: Prior data was consumed and no sequence is executing.
        unsafe {
            r.module_control.clear_fifos();
            io_fence();
            r.pointer_clear.write((1 << 8) | 1);
            io_fence();
        }
        let address = AMBA_BASE + self.config.chip_select as u32 * 0x400;
        self.retry_write(move || {
            // SAFETY: An idle controller uses SFAR solely for selecting this 1 KiB window.
            unsafe { r.flash_address.write(address) };
        })?;
        // SAFETY: Exclusive LUT ownership; replace and relock one idle sequence.
        unsafe {
            r.lut_key.write(0x5af0_5af0);
            io_fence();
            r.lut_control.write(2);
            io_fence();
            for (register, word) in r.lut[60..64].iter().zip(words) {
                register.write(word);
            }
            io_fence();
            r.lut_key.write(0x5af0_5af0);
            io_fence();
            r.lut_control.write(1);
            io_fence();
            r.flags.clear(IP_FLAGS);
            io_fence();
        }
        Ok(())
    }

    fn launch(&mut self, length: usize) -> Result<(), Error> {
        self.failed = true;
        // SAFETY: The LUT is complete, FIFO sizes checked and CS owned by this backend.
        unsafe {
            self.registers
                .ip_command
                .write((SEQUENCE << 24) | length as u32)
        };
        io_fence();
        for _ in 0..self.config.poll_budget {
            let flags = self.registers.flags.read();
            if flags & IP_ERROR != 0 {
                return Err(Error::IpCommand(flags));
            }
            if flags & TRANSFER_FINISHED != 0 {
                self.idle()?;
                io_fence();
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::PollLimit)
    }

    fn finish(&mut self, result: Result<(), Error>) -> Result<(), Error> {
        // Never reset an active controller; failed recovery keeps this backend unusable.
        let recovery = self.reset();
        self.failed = recovery.is_err();
        if !self.failed {
            // SAFETY: The sequence is idle and only handled IP flags are acknowledged.
            unsafe { self.registers.flags.clear(IP_FLAGS) };
            io_fence();
        }
        result.and(recovery)
    }
}

impl Backend for BlockingQspi<'_> {
    type Error = Error;
    // Leave one word unused to avoid the full-RX-FIFO alignment restriction.
    const MAX_READ_SIZE: usize = 124;
    const MAX_WRITE_SIZE: usize = 256;

    fn command(&mut self, transfer: Transfer) -> Result<(), Error> {
        self.prepare(lut::encode(transfer, None)?)?;
        let result = self.launch(0);
        self.finish(result)
    }

    fn read(&mut self, transfer: Transfer, data: &mut [u8]) -> Result<(), Error> {
        if data.len() > Self::MAX_READ_SIZE {
            return Err(Error::InvalidTransfer);
        }
        self.prepare(lut::encode(transfer, (!data.is_empty()).then_some(true))?)?;
        let result = self.launch(data.len());
        if result.is_ok() {
            for (register, chunk) in self.registers.receive_buffer.iter().zip(data.chunks_mut(4)) {
                chunk.copy_from_slice(&register.read().to_le_bytes()[..chunk.len()]);
            }
        }
        self.finish(result)
    }

    fn write(&mut self, transfer: Transfer, data: &[u8]) -> Result<(), Error> {
        if data.len() > Self::MAX_WRITE_SIZE {
            return Err(Error::InvalidTransfer);
        }
        self.prepare(lut::encode(transfer, (!data.is_empty()).then_some(false))?)?;
        for chunk in data.chunks(4) {
            let mut word = [0; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            // SAFETY: FIFO was cleared and the complete payload fits before launch.
            unsafe {
                self.registers
                    .transmit_buffer_data
                    .write(u32::from_le_bytes(word))
            };
            io_fence();
        }
        if !data.is_empty() {
            // SpacemiT requires at least 16 bytes present for a TX FIFO pop.
            for _ in data.len().div_ceil(4)..4 {
                // SAFETY: Padding fills the FIFO but IPCR transmits only the real length.
                unsafe { self.registers.transmit_buffer_data.write(0) };
                io_fence();
            }
        }
        let result = self.launch(data.len());
        self.finish(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoDelay;
    impl DelayNs for NoDelay {
        fn delay_ns(&mut self, _: u32) {
            panic!("recovery must not reset an active controller");
        }
    }

    #[test]
    fn failed_recovery_prevents_reset_and_further_commands() {
        // SAFETY: All fields accept zero; this is owned RAM, not live MMIO.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: The clock value and its volatile cell accept zero.
        let clock = unsafe { core::mem::zeroed() };
        let mut delay = NoDelay;
        let mut qspi = BlockingQspi {
            registers: &registers,
            _clock: &clock,
            delay: &mut delay,
            config: Config::new(Hertz(24_000_000)),
            failed: true,
        };
        // SAFETY: RAM records this bus write as XIP status; it does not emulate W1C.
        unsafe { registers.flags.clear(XIP) };
        assert_eq!(qspi.finish(Err(Error::PollLimit)), Err(Error::PollLimit));
        assert_eq!(registers.module_control.read_configuration(), 0);
        assert_eq!(
            qspi.command(Transfer::new(0x9f)),
            Err(Error::RecoveryRequired)
        );
        assert_eq!(
            qspi.read(Transfer::new(0x9f), &mut [0; 3]),
            Err(Error::RecoveryRequired)
        );
        assert_eq!(
            qspi.write(Transfer::new(0x01), &[0]),
            Err(Error::RecoveryRequired)
        );
    }
}
