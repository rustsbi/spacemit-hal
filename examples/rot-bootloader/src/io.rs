//! Locked standard output.

use crate::console::{self, Guard};
use embedded_io::{Error as _, ErrorKind, ErrorType, Write};
use spacemit_hal::uart::BlockingUart;

/// A console lock released on drop; unavailable handles report WriteZero.
#[must_use]
pub struct Stdout {
    guard: Option<Guard<'static, BlockingUart<'static>>>,
}

/// Tries to lock the initializing hart's UART until the handle is dropped.
pub fn stdout() -> Stdout {
    Stdout {
        guard: console::lock(),
    }
}

impl Stdout {
    fn uart(&mut self) -> Result<&mut BlockingUart<'static>, ErrorKind> {
        self.guard.as_deref_mut().ok_or(ErrorKind::WriteZero)
    }
}

impl ErrorType for Stdout {
    type Error = ErrorKind;
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, ErrorKind> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.uart()?.write(buf).map_err(|error| error.kind())
    }

    fn flush(&mut self) -> Result<(), ErrorKind> {
        self.uart()?.flush();
        Ok(())
    }
}
