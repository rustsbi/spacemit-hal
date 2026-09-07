//! NOR protocol and backend failures.

use embedded_storage::nor_flash::{NorFlashError, NorFlashErrorKind};

/// A NOR protocol or backend failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error<E> {
    /// The controller rejected or failed a transaction.
    Backend(E),
    /// The requested range exceeds the flash or serial address space.
    OutOfBounds,
    /// The JEDEC identification is absent or invalid.
    InvalidId,
    /// The SFDP header or basic parameter table is invalid.
    InvalidSfdp,
    /// Automatic probing cannot select a non-mutating read protocol.
    UnsupportedAddressing,
    /// The configuration or backend read limit is invalid.
    InvalidConfig,
    /// The flash remained busy for the configured status-read budget.
    PollLimit,
}

impl<E: core::fmt::Debug> NorFlashError for Error<E> {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Self::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            _ => NorFlashErrorKind::Other,
        }
    }
}
