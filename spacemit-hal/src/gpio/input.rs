use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, PinState};

use super::{Output, inner::GpioInner, k1, k3};

/// A digital input pin.
pub struct Input<'a> {
    inner: GpioInner<'a>,
}

impl<'a> Input<'a> {
    /// Converts this pin into a digital output.
    #[inline]
    pub fn into_output(self, initial_state: PinState) -> Output<'a> {
        Output::from_inner(self.inner, initial_state)
    }

    /// Temporarily selects output mode, restoring input on return or unwind.
    #[inline]
    pub fn with_output<F, T>(&mut self, initial_state: PinState, f: F) -> T
    where
        F: for<'b> FnOnce(&mut Output<'b>) -> T,
    {
        struct RestoreInput<'b>(GpioInner<'b>);
        impl Drop for RestoreInput<'_> {
            fn drop(&mut self) {
                self.0.configure_input();
            }
        }
        let _restore = RestoreInput(self.inner);
        let mut output = Output::from_inner(self.inner, initial_state);
        f(&mut output)
    }

    /// Constructs a K1/M1 input pin.
    ///
    /// # Safety
    /// Exclusively own this pin for 'a, with valid GPIO mappings, power, clocks
    /// and GPIO mux configuration; other code, harts and DMA must not control it.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k1(bank: u8, number: u8, gpio: &'a k1::RegisterBlock) -> Self {
        Self::from_inner(GpioInner::new_k1(bank, number, gpio))
    }

    /// Constructs a K3 input pin.
    ///
    /// # Safety
    /// Exclusively own this pin for 'a, with valid GPIO mappings, power, clocks
    /// and GPIO mux configuration; other code, harts and DMA must not control it.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k3(bank: u8, number: u8, gpio: &'a k3::RegisterBlock) -> Self {
        Self::from_inner(GpioInner::new_k3(bank, number, gpio))
    }

    #[inline]
    pub(super) fn from_inner(inner: GpioInner<'a>) -> Self {
        inner.configure_input();
        Self { inner }
    }
}

impl ErrorType for Input<'_> {
    type Error = Infallible;
}

impl InputPin for Input<'_> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.inner.is_high())
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.inner.is_high())
    }
}
