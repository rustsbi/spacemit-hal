//! Two-wire serial interfaces.

mod blocking;
mod pads;
mod register;
pub use blocking::{BlockingI2c, Config, Error};
pub use pads::{IntoI2c, IntoPads, Pads};
pub use register::{ControlRegister, DataBuffer, RegisterBlock};

/// An exclusive I²C controller paired with its clock identity.
///
/// # Safety
/// Transfer exclusive register access for 'a with valid mappings, power and
/// upstream bus access, no conflicting users or DMA, and no recreated owners.
/// The functional source is retained through `I2cFrequency` while a driver uses it.
pub unsafe trait Instance<'a> {
    /// The matching SoC-specific clock identity.
    type ClockId: crate::clock::I2cId;
    /// Consumes the controller or its mutable borrow.
    fn register_block(self) -> &'a RegisterBlock;
}

/// Constructs a polling I²C controller.
pub trait I2cExt<'a>: Instance<'a> + Sized {
    /// Enables standard-mode polling with matching pads and clock.
    #[inline]
    fn blocking(
        self,
        pads: impl IntoPads<'a, Self::ClockId>,
        clock: crate::clock::I2cFrequency<'a, Self::ClockId>,
        config: Config,
        delay: &mut impl embedded_hal::delay::DelayNs,
    ) -> Result<BlockingI2c<'a>, Error> {
        BlockingI2c::new(self, pads, clock, config, delay)
    }
}

impl<'a, U: Instance<'a>> I2cExt<'a> for U {}
