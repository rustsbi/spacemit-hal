use crate::{clock::UartId, gpio::FlexPad};

/// Routes a pad to the specified SoC-specific UART transmitter.
///
/// # Safety
/// The pad and selected mux must physically route to I's TX signal; conversion
/// must transfer exclusive pad access for 'a with valid mappings, power and clocks,
/// no conflicting users or DMA, and no recreated owner.
pub unsafe trait IntoTransmit<'a, I: UartId> {
    /// Selects TX routing and retains the pad resource.
    fn into_uart_transmit(self) -> FlexPad<'a>;
}

/// Routes a pad to the specified SoC-specific UART receiver.
///
/// # Safety
/// The pad and selected mux must physically route to I's RX signal; conversion
/// must transfer exclusive pad access for 'a with valid mappings, power and clocks,
/// no conflicting users or DMA, and no recreated owner.
pub unsafe trait IntoReceive<'a, I: UartId> {
    /// Selects RX routing and retains the pad resource.
    fn into_uart_receive(self) -> FlexPad<'a>;
}

mod sealed {
    pub trait Sealed {}
    impl<T, R> Sealed for (T, R) {}
}

/// A matching (TX, RX) pair for one SoC-specific UART.
pub trait Pads<'a, I: UartId>: sealed::Sealed {
    /// Configures both routes and retains their exclusive pad resources.
    fn into_uart_pads(self) -> (FlexPad<'a>, FlexPad<'a>);
}

impl<'a, I: UartId, T: IntoTransmit<'a, I>, R: IntoReceive<'a, I>> Pads<'a, I> for (T, R) {
    #[inline]
    fn into_uart_pads(self) -> (FlexPad<'a>, FlexPad<'a>) {
        (self.0.into_uart_transmit(), self.1.into_uart_receive())
    }
}
