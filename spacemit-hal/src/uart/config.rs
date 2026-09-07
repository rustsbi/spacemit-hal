//! UART frame and baud-rate configuration.

pub use embedded_time::rate::Baud;

/// UART frame and baud-rate configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// Requested baud rate, ignored only by raw BootROM adoption.
    pub baudrate: Baud,
    /// Number of data bits.
    pub wordlength: WordLength,
    /// Parity selection.
    pub parity: Parity,
    /// Number of stop bits.
    pub stopbits: StopBits,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            baudrate: Baud(115_200),
            wordlength: WordLength::Eight,
            parity: Parity::None,
            stopbits: StopBits::One,
        }
    }
}

/// UART data-bit count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordLength {
    /// Five bits.
    Five,
    /// Six bits.
    Six,
    /// Seven bits.
    Seven,
    /// Eight bits.
    Eight,
}

/// UART parity selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    /// No parity.
    None,
    /// Odd parity.
    Odd,
    /// Even parity.
    Even,
}

/// UART stop-bit count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopBits {
    /// One stop bit.
    One,
    /// Two stop bits, or 1.5 with five data bits.
    Two,
}
