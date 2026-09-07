/// A bootloader operation result.
pub type Result<T> = core::result::Result<T, Error>;

/// A bootloader operation failed.
#[derive(Debug)]
pub enum Error {
    /// Clock initialization failed.
    Clock(spacemit_hal::clock::Error),
    /// An I²C transaction failed.
    I2c(spacemit_hal::i2c::Error),
    /// An EEPROM operation failed.
    Eeprom(eeprom24x::Error<spacemit_hal::i2c::Error>),
    /// The EEPROM format is invalid.
    Format,
    /// The EEPROM checksum is invalid.
    Checksum,
    /// A PMIC operation failed.
    Pmic(spacemit_p1::Error<spacemit_hal::i2c::Error>),
    /// Platform counter initialization failed.
    Timer,
    /// The board console could not acquire its global slot.
    ConsoleUnavailable,
}

impl From<spacemit_hal::clock::Error> for Error {
    fn from(error: spacemit_hal::clock::Error) -> Self {
        Self::Clock(error)
    }
}

impl From<spacemit_hal::i2c::Error> for Error {
    fn from(error: spacemit_hal::i2c::Error) -> Self {
        Self::I2c(error)
    }
}

impl From<spacemit_hal::counter::Error> for Error {
    fn from(_: spacemit_hal::counter::Error) -> Self {
        Self::Timer
    }
}

impl From<spacemit_p1::Error<spacemit_hal::i2c::Error>> for Error {
    fn from(error: spacemit_p1::Error<spacemit_hal::i2c::Error>) -> Self {
        Self::Pmic(error)
    }
}
