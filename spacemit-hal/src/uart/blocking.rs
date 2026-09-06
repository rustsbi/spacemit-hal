//! Configurable polling UART I/O.

use super::{Config, Instance, Parity, RegisterBlock, StopBits, WordLength};
use core::{fmt, hint::spin_loop};
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
pub struct BlockingUart<'a> {
    uart: &'a RegisterBlock,
    pending_error: Error,
}

impl<'a> BlockingUart<'a> {
    /// Configures polling and resets FIFOs while retaining the current baud rate.
    ///
    /// # Safety
    /// The registers must refer to a live K1/M1 or K3 UART with its clocks,
    /// reset, pads, and baud rate configured; transfers must be idle and any
    /// external DMA engine stopped. No other code,
    /// hart, or operating-system driver may access this UART until it is freed
    /// or dropped, and its mapping and configuration must remain valid.
    pub unsafe fn new(uart: impl Instance<'a>, config: Config) -> Self {
        let uart = uart.register_block();
        use uart16550::{CharLen, LineControl};
        // TODO: Program config.baudrate once the UART input clock is supplied.
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
            pending_error: Error {
                overrun: false,
                parity: false,
                framing: false,
                break_detected: false,
            },
        }
    }

    /// Returns the register block without flushing or changing configuration.
    pub fn free(self) -> &'a RegisterBlock {
        self.uart
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
            unsafe { BlockingUart::new(self.registers(), Config::default()) }
        }
    }

    #[test]
    fn constructor_configures_polling_and_free_retains_it() {
        let fake = FakeUart::new(0);
        fake.set(1, 0xff);
        fake.set(3, 0xff);
        let uart = fake.uart();
        assert!(core::ptr::eq(uart.free(), fake.registers()));
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
                    let fake = FakeUart::new(0);
                    fake.set(0, 0xab);
                    fake.set(3, 0xff);
                    fake.set(7, 0x1234);
                    // SAFETY: Exclusive sequential access to an aligned RAM fixture.
                    let uart = unsafe {
                        BlockingUart::new(
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
                    let _ = uart.free();
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
