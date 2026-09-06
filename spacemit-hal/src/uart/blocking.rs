//! Configurable polling UART I/O.

use super::{ClockedInstance, Config, Instance, Parity, RegisterBlock, StopBits, WordLength};
use crate::clock::{self, Hertz, UartClock, UartClockRef};
use core::{fmt, hint::spin_loop, marker::PhantomData};
use uart16550::LineStatus;

/// Receive faults accumulated from the line-status register.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Error {
    /// The receiver lost data because its buffer was full.
    pub overrun: bool,
    /// A received character failed parity checking.
    pub parity: bool,
    /// A received character had an invalid stop bit.
    pub framing: bool,
    /// The receiver detected a break condition.
    pub break_detected: bool,
}

impl Error {
    fn capture(&mut self, status: LineStatus) {
        self.overrun |= status.is_overrun_error();
        self.parity |= status.is_parity_error();
        self.framing |= status.is_framing_error();
        self.break_detected |= status.is_break_condition();
    }

    fn is_present(self) -> bool {
        self.overrun || self.parity || self.framing || self.break_detected
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UART receive error: {self:?}")
    }
}

impl core::error::Error for Error {}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        if self.overrun {
            embedded_io::ErrorKind::Other
        } else {
            embedded_io::ErrorKind::InvalidData
        }
    }
}

/// An exclusively borrowed UART using polling without timeouts.
#[must_use]
pub struct BlockingUart<'a> {
    uart: &'a RegisterBlock,
    clock: Option<&'a UartClockRef<'a>>,
    pending_error: Error,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<'a> BlockingUart<'a> {
    /// Checks a matching clock token and configures polling at the existing baud.
    ///
    /// # Safety
    /// UART mapping, power, pads, and divisor must be valid for 'a; transfers
    /// must be idle and external DMA stopped. Exclusive UART and clock-source
    /// access must remain valid for the borrow. Dropping or forgetting the driver
    /// does not stop transfers; wait for TX completion before reconfiguration.
    /// This function does not freeze upstream clocks.
    pub unsafe fn new<U: ClockedInstance<'a>>(
        uart: U,
        clock: &'a mut UartClock<'_, U::ClockId>,
        config: Config,
    ) -> Result<Self, clock::Error> {
        let clock = clock.borrow();
        clock.check()?;
        // Erase the instance types only after matching them, retaining both borrows.
        Ok(Self::configure(uart.register_block(), Some(clock), config))
    }

    /// Configures polling and resets FIFOs while retaining the current baud rate.
    ///
    /// # Safety
    /// The registers must refer to a live K1/M1 or K3 UART with its clocks,
    /// reset, pads, and baud rate configured; transfers must be idle and any
    /// external DMA engine stopped. No other code,
    /// hart, or operating-system driver may access this UART for the borrow,
    /// and its mapping and clock configuration must remain valid. Dropping or
    /// forgetting the driver does not stop transfers; wait for TX completion
    /// before reconfiguration.
    pub unsafe fn from_bootrom(uart: impl Instance<'a>, config: Config) -> Self {
        Self::configure(uart.register_block(), None, config)
    }

    fn configure(
        uart: &'a RegisterBlock,
        clock: Option<&'a UartClockRef<'a>>,
        config: Config,
    ) -> Self {
        use uart16550::{CharLen, LineControl};
        // TODO: Program config.baudrate after validating a known input frequency
        // and divisor error; adoption may deliberately retain an unknown rate.
        let length = match config.wordlength {
            WordLength::Five => CharLen::FIVE,
            WordLength::Six => CharLen::SIX,
            WordLength::Seven => CharLen::SEVEN,
            WordLength::Eight => CharLen::EIGHT,
        };
        let parity = match config.parity {
            Parity::None => 0,
            Parity::Odd => 0x08,
            Parity::Even => 0x18,
        };
        uart.configure_polling(
            LineControl::default()
                .set_char_len(length)
                .set_one_stop_bit(matches!(config.stopbits, StopBits::One))
                .disable_dlr_access(),
            parity,
        );
        Self {
            uart,
            clock,
            pending_error: Error {
                overrun: false,
                parity: false,
                framing: false,
                break_detected: false,
            },
            _not_send_sync: PhantomData,
        }
    }

    /// Returns the supplied UART input frequency, or None when unknown.
    pub fn input_clock(&self) -> Option<Hertz> {
        self.clock.and_then(UartClockRef::frequency)
    }

    fn status(&mut self) -> LineStatus {
        let status = self.uart.lsr().read();
        // LSR reads can clear receive errors, including during TX polling.
        // Preserve them until read_byte reports them, like Linux's lsr_saved_flags.
        self.pending_error.capture(status);
        status
    }

    /// Waits for a byte, discarding the FIFO head when reporting receive faults.
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        loop {
            let status = self.status();
            if self.pending_error.is_present() {
                if status.is_data_ready() {
                    let _ = self.uart.rbr_thr().rx_data();
                }
                return Err(core::mem::take(&mut self.pending_error));
            }
            if status.is_data_ready() {
                return Ok(self.uart.rbr_thr().rx_data());
            }
            spin_loop();
        }
    }

    /// Waits for transmitter space and sends one byte.
    pub fn write_byte(&mut self, byte: u8) {
        while !self.status().is_transmitter_fifo_empty() {
            spin_loop();
        }
        self.uart.rbr_thr().tx_data(byte);
    }

    /// Waits until both the transmit FIFO and shift register are empty.
    pub fn flush(&mut self) {
        while !self.status().is_transmitter_empty() {
            spin_loop();
        }
    }
}

impl embedded_io::ErrorType for BlockingUart<'_> {
    type Error = Error;
}

impl embedded_io::Read for BlockingUart<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let Some(first) = buf.first_mut() else {
            return Ok(0);
        };
        *first = self.read_byte()?;
        Ok(1)
    }
}

impl embedded_io::Write for BlockingUart<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for &byte in buf {
            self.write_byte(byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        BlockingUart::flush(self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::UnsafeCell;
    use embedded_io::{Read, Write};

    enum TestUart0 {}
    enum TestUart2 {}
    // SAFETY: These markers identify distinct RAM-backed UARTs in each fixture.
    unsafe impl clock::UartId for TestUart0 {}
    // SAFETY: TestUart2 never identifies TestUart0's simulated registers.
    unsafe impl clock::UartId for TestUart2 {}

    struct TestInstance<'a, I>(&'a RegisterBlock, PhantomData<I>);

    impl<'a, I> Instance<'a> for TestInstance<'a, I> {
        fn register_block(self) -> &'a RegisterBlock {
            self.0
        }
    }

    // SAFETY: Tests pair each identity with its exclusive initialized RAM fixture;
    // erasure retains that fixture's borrow and performs no teardown.
    unsafe impl<'a, I: clock::UartId> ClockedInstance<'a> for TestInstance<'a, I> {
        type ClockId = I;
    }

    fn clock_register(bits: u32) -> volatile_register::RW<crate::apbc::UartClockReset> {
        // SAFETY: The wrapper is transparent over an initialized integer cell.
        let register: volatile_register::RW<crate::apbc::UartClockReset> =
            unsafe { core::mem::zeroed() };
        // SAFETY: Exclusive initialization of a RAM-backed register.
        unsafe { register.write(crate::apbc::UartClockReset::from_bits(bits)) };
        register
    }

    #[repr(C)]
    struct FakeUart([UnsafeCell<u32>; 8]);

    impl FakeUart {
        fn new(lsr: u32) -> Self {
            let fake = Self(core::array::from_fn(|_| UnsafeCell::new(0)));
            fake.set(5, lsr);
            fake
        }

        fn set(&self, index: usize, value: u32) {
            // SAFETY: This test owns initialized RAM and has no concurrent accesses.
            unsafe { self.0[index].get().write(value) };
        }

        fn get(&self, index: usize) -> u32 {
            // SAFETY: This test owns initialized RAM and has no concurrent accesses.
            unsafe { self.0[index].get().read() }
        }

        fn registers(&self) -> &RegisterBlock {
            // SAFETY: The UART block is eight repr(transparent) UnsafeCell<u32>
            // fields in a repr(C) block, matching this live, aligned RAM array.
            unsafe { &*self.0.as_ptr().cast::<RegisterBlock>() }
        }

        fn uart(&self) -> BlockingUart<'_> {
            // SAFETY: Tests use this RAM-backed UART sequentially, never hardware.
            unsafe { BlockingUart::from_bootrom(self.registers(), Config::default()) }
        }
    }

    #[test]
    fn generic_constructors_return_one_erased_driver_type() {
        let first = FakeUart::new(0x60);
        let second = FakeUart::new(0x60);
        let gate0 = clock_register(3);
        let gate2 = clock_register(0x13);
        let frequencies =
            clock::Clocks::new(Some(Hertz(57_600_000)), Some(Hertz(14_745_600)), None).unwrap();
        // SAFETY: Each local token exclusively owns its simulated clock register.
        let mut clock0 = unsafe { UartClock::from_register(&gate0, frequencies) };
        // SAFETY: The second clock is disjoint from the first.
        let mut clock2 = unsafe { UartClock::from_register(&gate2, frequencies) };
        // SAFETY: Independent simulated UARTs, stable clock cells, no DMA or aliases.
        let first = unsafe {
            BlockingUart::new(
                TestInstance::<TestUart0>(first.registers(), PhantomData),
                &mut clock0,
                Config::default(),
            )
            .unwrap()
        };
        // SAFETY: The second fixture and gate are disjoint from the first.
        let second = unsafe {
            BlockingUart::new(
                TestInstance::<TestUart2>(second.registers(), PhantomData),
                &mut clock2,
                Config::default(),
            )
            .unwrap()
        };
        let mut ports: [BlockingUart<'_>; 2] = [first, second];
        assert_eq!(ports[0].input_clock(), Some(Hertz(57_600_000)));
        assert_eq!(ports[1].input_clock(), Some(Hertz(14_745_600)));
        ports[0].write_byte(b'a');
        ports[1].write_byte(b'b');
        assert_eq!(gate0.read().bits(), 3);
        assert_eq!(gate2.read().bits(), 0x13);
    }

    #[test]
    fn invalid_clock_is_rejected_before_touching_uart_and_resources_survive() {
        let fake = FakeUart::new(0x60);
        fake.set(1, 0x55);
        fake.set(2, 0x66);
        fake.set(3, 0x77);
        let gate = clock_register(0);
        // SAFETY: The local token exclusively owns the simulated clock register.
        let mut clock = unsafe { UartClock::from_register(&gate, clock::Clocks::unknown()) };
        // SAFETY: The registers are valid RAM even with the simulated clock off;
        // the driver must not access them until its clock check succeeds.
        let error = unsafe {
            BlockingUart::new(
                TestInstance::<TestUart0>(fake.registers(), PhantomData),
                &mut clock,
                Config::default(),
            )
            .err()
            .unwrap()
        };
        assert_eq!(error, clock::Error::Disabled);
        assert_eq!([fake.get(1), fake.get(2), fake.get(3)], [0x55, 0x66, 0x77]);
        // SAFETY: Emulate completion of platform clock recovery in this RAM fixture.
        unsafe { gate.write(crate::apbc::UartClockReset::from_bits(3)) };
        // SAFETY: Recovery restored the simulated clock; the original token is reused.
        let mut uart = unsafe {
            BlockingUart::new(
                TestInstance::<TestUart0>(fake.registers(), PhantomData),
                &mut clock,
                Config::default(),
            )
            .unwrap()
        };
        assert_eq!(uart.input_clock(), None);
        assert_eq!(fake.get(3), 3);
        uart.flush();
        assert_eq!(gate.read().bits(), 3);
    }

    #[test]
    fn clock_outlives_driver_and_can_be_borrowed_again() {
        let fake = FakeUart::new(0x60);
        let gate = clock_register(3);
        // SAFETY: The local token exclusively owns the simulated clock register.
        let mut clock = unsafe { UartClock::from_register(&gate, clock::Clocks::unknown()) };
        for &byte in b"xy" {
            // SAFETY: Sequential borrows of idle RAM fixtures without DMA or IRQs.
            let mut uart = unsafe {
                BlockingUart::new(
                    TestInstance::<TestUart0>(fake.registers(), PhantomData),
                    &mut clock,
                    Config::default(),
                )
                .unwrap()
            };
            uart.write_byte(byte);
            uart.flush();
        }
        assert_eq!(clock.frequency(), None);
        assert_eq!(gate.read().bits(), 3);
        assert_eq!(fake.get(0), u32::from(b'y'));
    }

    #[test]
    fn drop_and_forget_do_not_write_clock_registers() {
        let fake = FakeUart::new(0x20);
        let gate = clock_register(3);
        // SAFETY: This token outlives both borrows of the simulated clock register.
        let mut clock = unsafe { UartClock::from_register(&gate, clock::Clocks::unknown()) };
        {
            // SAFETY: The simulated UART and clock are exclusively owned here.
            let mut uart = unsafe {
                BlockingUart::new(
                    TestInstance::<TestUart0>(fake.registers(), PhantomData),
                    &mut clock,
                    Config::default(),
                )
                .unwrap()
            };
            uart.write_byte(b'x');
            // TX is still busy: leaving scope must neither wait nor disable clocks.
        }
        assert_eq!(gate.read().bits(), 3);
        // Simulate hardware finishing TX before borrowing the original tokens again.
        fake.set(5, 0x60);
        // SAFETY: The original fixtures are idle and remain exclusively owned.
        let uart = unsafe {
            BlockingUart::new(
                TestInstance::<TestUart0>(fake.registers(), PhantomData),
                &mut clock,
                Config::default(),
            )
            .unwrap()
        };
        // Keep this regression check if a future version adds a destructor.
        #[allow(clippy::forget_non_drop)]
        core::mem::forget(uart);
        assert_eq!(clock.frequency(), None);
        assert_eq!(gate.read().bits(), 3);
    }

    #[test]
    fn constructor_configures_polling() {
        let fake = FakeUart::new(0x60);
        fake.set(1, 0xff);
        fake.set(3, 0xff);
        let uart = fake.uart();
        assert!(core::ptr::eq(uart.uart, fake.registers()));
        assert_eq!(fake.get(1), 0x40);
        assert_eq!(fake.get(3), 0x03);
        assert_eq!(fake.get(2), 0x07);
    }

    #[test]
    fn constructor_maps_all_frame_formats() {
        for (wordlength, bits) in [
            (WordLength::Five, 0),
            (WordLength::Six, 1),
            (WordLength::Seven, 2),
            (WordLength::Eight, 3),
        ] {
            for (parity, parity_bits) in [(Parity::None, 0), (Parity::Odd, 8), (Parity::Even, 24)] {
                for (stopbits, stop_bit) in [(StopBits::One, 0), (StopBits::Two, 4)] {
                    let fake = FakeUart::new(0x60);
                    fake.set(0, 0xab);
                    fake.set(3, 0xff);
                    fake.set(7, 0x1234);
                    // SAFETY: Exclusive sequential access to an aligned RAM fixture.
                    let _uart = unsafe {
                        BlockingUart::from_bootrom(
                            fake.registers(),
                            Config {
                                wordlength,
                                parity,
                                stopbits,
                                ..Config::default()
                            },
                        )
                    };
                    assert_eq!(fake.get(3), bits | parity_bits | stop_bit);
                    assert_eq!(fake.get(1), 0x40);
                    assert_eq!(fake.get(2), 7);
                    assert_eq!(fake.get(0), 0xab);
                    assert_eq!(fake.get(7), 0x1234);
                }
            }
        }
    }

    #[test]
    fn empty_buffers_do_not_wait_or_consume_status() {
        let fake = FakeUart::new(0x1e);
        let mut uart = fake.uart();
        assert_eq!(uart.read(&mut []), Ok(0));
        assert_eq!(uart.write(&[]), Ok(0));
        assert!(!uart.pending_error.is_present());
    }

    #[test]
    fn read_returns_one_byte_without_waiting_to_fill_buffer() {
        let fake = FakeUart::new(0x01);
        fake.set(0, 0xa5);
        let mut uart = fake.uart();
        let mut buf = [0; 4];
        assert_eq!(uart.read(&mut buf), Ok(1));
        assert_eq!(buf, [0xa5, 0, 0, 0]);
    }

    #[test]
    fn write_and_flush_use_standard_line_status() {
        let fake = FakeUart::new(0x60);
        let mut uart = fake.uart();
        assert_eq!(uart.write(b"abc"), Ok(3));
        assert_eq!(fake.get(0), u32::from(b'c'));
        assert_eq!(Write::flush(&mut uart), Ok(()));
        fake.set(5, 0x20);
        let status = uart.status();
        assert!(status.is_transmitter_fifo_empty());
        assert!(!status.is_transmitter_empty());
    }

    #[test]
    fn transmit_polling_preserves_receive_errors() {
        let fake = FakeUart::new(0x20 | 0x06);
        let mut uart = fake.uart();
        uart.write_byte(b'x');
        // Emulate the hardware clearing error flags after an LSR read.
        fake.set(5, 0x01);
        assert_eq!(
            uart.read_byte(),
            Err(Error {
                overrun: true,
                parity: true,
                ..Error::default()
            })
        );
        assert!(!uart.pending_error.is_present());
    }

    #[test]
    fn reports_each_receive_error() {
        for (bit, expected) in [
            (
                0x02,
                Error {
                    overrun: true,
                    ..Error::default()
                },
            ),
            (
                0x04,
                Error {
                    parity: true,
                    ..Error::default()
                },
            ),
            (
                0x08,
                Error {
                    framing: true,
                    ..Error::default()
                },
            ),
            (
                0x10,
                Error {
                    break_detected: true,
                    ..Error::default()
                },
            ),
        ] {
            let fake = FakeUart::new(bit | 0x01);
            assert_eq!(fake.uart().read_byte(), Err(expected));
        }
    }
}
