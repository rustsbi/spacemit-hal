use crate::{clock::I2cId, gpio::FlexPad};
use core::marker::PhantomData;

/// Configures a supported I²C pin's mux and electrical settings.
pub trait IntoI2c<'a> {
    /// Selects I²C mode and retains the pad.
    fn into_i2c(self) -> FlexPad<'a>;
}

/// Owned or borrowed I²C pads with erased identities.
pub struct Pads<'a> {
    _borrow: PhantomData<(FlexPad<'a>, &'a mut ())>,
}

impl<'a> Pads<'a> {
    /// Retains configured GPIO pads.
    #[inline]
    pub fn from_gpio(_scl: FlexPad<'a>, _sda: FlexPad<'a>) -> Self {
        Self {
            _borrow: PhantomData,
        }
    }

    /// Retains exclusive access to dedicated I²C pads.
    ///
    /// # Safety
    /// Transfer configured dedicated pads for 'a without recreating owners;
    /// mappings and power must remain valid, with no conflicting users or DMA.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __dedicated() -> Self {
        Self {
            _borrow: PhantomData,
        }
    }
}

/// Configures the matching controller's SCL/SDA pads.
///
/// # Safety
/// Return the exact I²C route with exclusive access for 'a and valid mappings,
/// power and electrical settings, without conflicting users, DMA or recreated owners.
pub unsafe trait IntoPads<'a, I: I2cId> {
    /// Configures and retains both pads.
    fn into_i2c_pads(self) -> Pads<'a>;
}
