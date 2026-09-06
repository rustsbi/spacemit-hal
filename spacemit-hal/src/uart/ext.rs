use super::{BlockingUart, ClockedInstance, Config, Pads};
use crate::clock::{self, UartClock};

/// Constructs a polling UART from an owned or mutably borrowed instance.
pub trait UartExt<'a>: ClockedInstance<'a> + Sized {
    /// Configures TX/RX pads and polling, resetting FIFOs at the existing baud.
    fn blocking(
        self,
        pads: impl Pads<'a, Self::ClockId>,
        clock: &'a mut UartClock<'_, Self::ClockId>,
        config: Config,
    ) -> Result<BlockingUart<'a>, clock::Error> {
        BlockingUart::new(self, pads, clock, config)
    }
}

impl<'a, U: ClockedInstance<'a>> UartExt<'a> for U {}
