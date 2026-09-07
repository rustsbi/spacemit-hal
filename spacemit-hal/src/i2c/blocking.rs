use super::{Instance, IntoPads, Pads, RegisterBlock};
use crate::clock::{I2cFrequency, I2cFrequencyRef, io_fence};
use embedded_hal::{
    delay::DelayNs,
    i2c::{self, NoAcknowledgeSource, Operation},
};

// SpacemiT spacemit_i2c.h and K3 user manual 14.7 (shared byte-mode interface).
const BASE: u32 = (1 << 13) | (1 << 14) | (1 << 21);
const START: u32 = 1;
const STOP: u32 = 2;
const NACK: u32 = 4;
const TRANSFER: u32 = 8;
const ABORT: u32 = 1 << 12;
const BUSY: u32 = (1 << 15) | (1 << 16);
const EVENTS: u32 = 0x01fc_0000;

/// Standard-mode polling configuration with retained hardware timing counts.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Maximum status reads per wait, not a time duration.
    pub poll_budget: u32,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            poll_budget: 100_000,
        }
    }
}

/// I²C initialization or transfer failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The address or polling budget is invalid.
    InvalidConfig,
    /// A wait exhausted its polling budget.
    PollLimit,
    /// Another master won arbitration.
    ArbitrationLost,
    /// The addressed device rejected an address or byte.
    NoAcknowledge(NoAcknowledgeSource),
    /// The controller detected a bus error.
    Bus,
    /// The bus needs platform recovery before reuse.
    RecoveryRequired,
}

impl i2c::Error for Error {
    #[inline]
    fn kind(&self) -> i2c::ErrorKind {
        match *self {
            Self::ArbitrationLost => i2c::ErrorKind::ArbitrationLoss,
            Self::NoAcknowledge(source) => i2c::ErrorKind::NoAcknowledge(source),
            Self::Bus => i2c::ErrorKind::Bus,
            _ => i2c::ErrorKind::Other,
        }
    }
}

/// A polling I²C controller retaining its pads, gate and shared-source borrows.
pub struct BlockingI2c<'a> {
    registers: &'a RegisterBlock,
    _clock: core::marker::PhantomData<I2cFrequencyRef<'a>>,
    _pads: Pads<'a>,
    budget: u32,
    failed: bool,
}

impl<'a> BlockingI2c<'a> {
    /// Enables standard-mode polling with matching pads and clock.
    #[inline]
    pub fn new<U: Instance<'a>>(
        i2c: U,
        pads: impl IntoPads<'a, U::ClockId>,
        mut clock: I2cFrequency<'a, U::ClockId>,
        config: Config,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        if config.poll_budget == 0 {
            return Err(Error::InvalidConfig);
        }
        clock.enable(delay);
        let pads = pads.into_i2c_pads();
        let this = Self {
            registers: i2c.register_block(),
            _clock: core::marker::PhantomData,
            _pads: pads,
            budget: config.poll_budget,
            failed: false,
        };
        this.idle()?;
        this.command(1 << 10);
        delay.delay_us(100);
        // SAFETY: Exclusive idle controller; acknowledge only W1C byte events.
        unsafe {
            this.registers.slave_address.write(0);
            this.registers.status.clear(EVENTS);
        }
        io_fence();
        this.command(BASE);
        delay.delay_us(10);
        Ok(this)
    }

    #[inline]
    fn command(&self, bits: u32) {
        // SAFETY: The driver owns the live controller and writes a complete command.
        unsafe { self.registers.control.write(bits) };
        io_fence();
    }

    #[inline]
    fn idle(&self) -> Result<(), Error> {
        for _ in 0..self.budget {
            if self.registers.status.read() & BUSY == 0 {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::PollLimit)
    }

    #[inline]
    fn byte(&mut self, value: Option<u8>, flags: u32) -> Result<u8, Error> {
        let done = if value.is_some() { 1 << 19 } else { 1 << 20 };
        // SAFETY: Previous byte completed; clear stale events and fill TX when needed.
        unsafe {
            self.registers.status.clear(EVENTS);
            if let Some(value) = value {
                self.registers.data_buffer.write(value);
            }
        }
        io_fence();
        self.command(BASE | flags | TRANSFER);
        for _ in 0..self.budget {
            let status = self.registers.status.read();
            if let Some(error) = status_error(status, value.is_some(), flags & START != 0) {
                return Err(error);
            }
            if status & done != 0 {
                io_fence();
                let value = if value.is_none() {
                    self.registers.data_buffer.read()
                } else {
                    0
                };
                // SAFETY: Acknowledge exactly the consumed byte's event.
                unsafe { self.registers.status.clear(done) };
                io_fence();
                return Ok(value);
            }
            core::hint::spin_loop();
        }
        Err(Error::PollLimit)
    }
}

#[inline]
fn status_error(status: u32, transmit: bool, address: bool) -> Option<Error> {
    if status & (1 << 18) != 0 {
        return Some(Error::ArbitrationLost);
    }
    if transmit && status & (1 << 14) != 0 && status & ((1 << 19) | (1 << 22)) != 0 {
        return Some(Error::NoAcknowledge(if address {
            NoAcknowledgeSource::Address
        } else {
            NoAcknowledgeSource::Data
        }));
    }
    (status & (1 << 22) != 0).then_some(Error::Bus)
}

impl i2c::ErrorType for BlockingI2c<'_> {
    type Error = Error;
}

impl i2c::I2c for BlockingI2c<'_> {
    #[inline]
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        if address > 0x7f {
            return Err(Error::InvalidConfig);
        }
        if self.failed {
            return Err(Error::RecoveryRequired);
        }
        if operations.iter().all(empty) {
            return Ok(());
        }
        self.idle()?;
        let result = transfer(address, operations, |byte, flags| self.byte(byte, flags))
            .and_then(|()| self.idle());
        if let Err(error) = result {
            // MA must not interrupt an active byte or another master's transaction.
            if error != Error::ArbitrationLost && self.registers.control.read() & TRANSFER == 0 {
                self.command(BASE | ABORT);
            }
            self.failed = self.idle().is_err() || self.registers.control.read() & TRANSFER != 0;
            if !self.failed {
                self.command(BASE);
            }
        }
        result
    }
}

#[inline]
fn empty(operation: &Operation<'_>) -> bool {
    match operation {
        Operation::Read(b) => b.is_empty(),
        Operation::Write(b) => b.is_empty(),
    }
}

#[inline]
fn transfer(
    address: u8,
    operations: &mut [Operation<'_>],
    mut byte: impl FnMut(Option<u8>, u32) -> Result<u8, Error>,
) -> Result<(), Error> {
    let mut previous = None;
    for index in 0..operations.len() {
        if empty(&operations[index]) {
            continue;
        }
        let read = matches!(operations[index], Operation::Read(_));
        let next = operations[index + 1..]
            .iter()
            .find(|op| !empty(op))
            .map(|op| matches!(op, Operation::Read(_)));
        if previous != Some(read) {
            byte(Some((address << 1) | u8::from(read)), START)?;
        }
        match &mut operations[index] {
            Operation::Write(bytes) => {
                for (i, &value) in bytes.iter().enumerate() {
                    byte(
                        Some(value),
                        if i + 1 == bytes.len() && next.is_none() {
                            STOP
                        } else {
                            0
                        },
                    )?;
                }
            }
            Operation::Read(bytes) => {
                let len = bytes.len();
                for (i, value) in bytes.iter_mut().enumerate() {
                    let last = i + 1 == len;
                    let flags = if last && next != Some(true) { NACK } else { 0 }
                        | if last && next.is_none() { STOP } else { 0 };
                    *value = byte(None, flags)?;
                }
            }
        }
        previous = Some(read);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::{Clocks, I2cClock};
    extern crate std;
    use std::vec::Vec;

    struct Token<'a>(&'a RegisterBlock);
    enum TestId {}
    // SAFETY: Only the initialization test accesses this zero-valid RAM fixture.
    static mut GATE: volatile_register::WO<crate::apbc::TwsiClockReset> =
        unsafe { core::mem::zeroed() };
    // SAFETY: This test identity names one initialized RAM controller and clock.
    unsafe impl crate::clock::I2cId for TestId {
        const CLOCK_REGISTER: *const volatile_register::WO<crate::apbc::TwsiClockReset> =
            core::ptr::addr_of!(GATE);
    }
    // SAFETY: Each test uses disjoint RAM fixtures without concurrent users or DMA.
    unsafe impl<'a> Instance<'a> for Token<'a> {
        type ClockId = TestId;
        fn register_block(self) -> &'a RegisterBlock {
            self.0
        }
    }
    struct TestPads;
    // SAFETY: Test-only dedicated pads have no physical side effects or other owners.
    unsafe impl<'a> IntoPads<'a, TestId> for TestPads {
        fn into_i2c_pads(self) -> Pads<'a> {
            // SAFETY: Exclusive simulated dedicated pads.
            unsafe { Pads::__dedicated() }
        }
    }
    struct Delay;
    impl DelayNs for Delay {
        fn delay_ns(&mut self, _: u32) {}
    }

    #[test]
    fn initialization_rejects_invalid_budget_before_writes() {
        // SAFETY: Only zero-valid integer register cells, never physical MMIO.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: This fixture is private to this test, never physical MMIO.
        let gate = unsafe { &*core::ptr::addr_of!(GATE) };
        // SAFETY: Exclusive test clock with no hardware dependencies.
        let mut clock = unsafe { I2cClock::<TestId>::__new() };
        let clocks = Clocks::for_test(true);
        let result = BlockingI2c::new(
            Token(&registers),
            TestPads,
            clock.with_clock(&clocks).unwrap(),
            Config { poll_budget: 0 },
            &mut Delay,
        );
        assert!(matches!(result, Err(Error::InvalidConfig)));
        assert_eq!(registers.control.read(), 0);
        // SAFETY: Inspect the initialized RAM fixture, not write-only hardware.
        assert_eq!(
            unsafe { core::ptr::from_ref(gate).cast::<u32>().read_volatile() },
            0
        );
        drop(result);
        let driver = BlockingI2c::new(
            Token(&registers),
            TestPads,
            clock.with_clock(&clocks).unwrap(),
            Config::default(),
            &mut Delay,
        )
        .unwrap();
        assert_eq!(driver.registers.control.read(), BASE);
        assert_eq!(driver.registers.slave_address.read(), 0);
        // SAFETY: Only a RAM fixture, where the WO cell records its last write.
        let value = unsafe { core::ptr::from_ref(gate).cast::<u32>().read_volatile() };
        assert_eq!(value, 3);
    }

    #[test]
    fn combined_transaction() {
        let (mut a, mut b) = ([0; 1], [0; 2]);
        let mut trace = Vec::new();
        transfer(
            0x50,
            &mut [
                Operation::Write(&[0]),
                Operation::Write(&[1]),
                Operation::Read(&mut a),
                Operation::Read(&mut []),
                Operation::Read(&mut b),
                Operation::Write(&[2]),
            ],
            |value, flags| {
                trace.push((value, flags));
                Ok(42)
            },
        )
        .unwrap();
        assert_eq!(
            trace,
            [
                (Some(0xa0), START),
                (Some(0), 0),
                (Some(1), 0),
                (Some(0xa1), START),
                (None, 0),
                (None, 0),
                (None, NACK),
                (Some(0xa0), START),
                (Some(2), STOP)
            ]
        );
        assert_eq!((a, b), ([42], [42, 42]));
    }

    #[test]
    fn final_read_and_empty_operations() {
        let mut trace = Vec::new();
        transfer(
            0x41,
            &mut [Operation::Read(&mut [0]), Operation::Write(&[])],
            |v, f| {
                trace.push((v, f));
                Ok(0)
            },
        )
        .unwrap();
        assert_eq!(trace, [(Some(0x83), START), (None, NACK | STOP)]);
        transfer(0, &mut [], |_, _| panic!()).unwrap();
    }

    #[test]
    fn errors_stop_progress() {
        let mut count = 0;
        let result = transfer(0x50, &mut [Operation::Write(&[1, 2])], |_, _| {
            count += 1;
            Err(Error::ArbitrationLost)
        });
        assert_eq!(result, Err(Error::ArbitrationLost));
        assert_eq!(count, 1);
        assert_eq!(
            status_error((1 << 19) | (1 << 14), true, true),
            Some(Error::NoAcknowledge(NoAcknowledgeSource::Address))
        );
        assert_eq!(
            status_error((1 << 22) | (1 << 14), true, false),
            Some(Error::NoAcknowledge(NoAcknowledgeSource::Data))
        );
        assert_eq!(status_error(1 << 22, false, false), Some(Error::Bus));
        assert_eq!(
            status_error(1 << 18, true, false),
            Some(Error::ArbitrationLost)
        );
    }
}
