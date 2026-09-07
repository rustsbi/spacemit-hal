use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

use super::{Input, inner::GpioInner, k1, k3};

/// A digital output pin.
pub struct Output<'a> {
    inner: GpioInner<'a>,
    state: PinState,
}

impl<'a> Output<'a> {
    /// Converts this pin into a digital input.
    #[inline]
    pub fn into_input(self) -> Input<'a> {
        Input::from_inner(self.inner)
    }

    /// Temporarily selects input mode, restoring output on return or unwind.
    #[inline]
    pub fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: for<'b> FnOnce(&mut Input<'b>) -> T,
    {
        struct RestoreOutput<'b>(GpioInner<'b>, PinState);
        impl Drop for RestoreOutput<'_> {
            #[inline]
            fn drop(&mut self) {
                self.0.configure_output(self.1);
            }
        }
        let _restore = RestoreOutput(self.inner, self.state);
        let mut input = Input::from_inner(self.inner);
        f(&mut input)
    }

    /// Constructs a K1/M1 output pin.
    ///
    /// # Safety
    /// Exclusively own this pin for 'a, with valid GPIO mappings, power, clocks
    /// and GPIO mux configuration; other code, harts and DMA must not control it.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k1(
        bank: u8,
        number: u8,
        gpio: &'a k1::RegisterBlock,
        initial_state: PinState,
    ) -> Self {
        Self::from_inner(GpioInner::new_k1(bank, number, gpio), initial_state)
    }

    /// Constructs a K3 output pin.
    ///
    /// # Safety
    /// Exclusively own this pin for 'a, with valid GPIO mappings, power, clocks
    /// and GPIO mux configuration; other code, harts and DMA must not control it.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k3(
        bank: u8,
        number: u8,
        gpio: &'a k3::RegisterBlock,
        initial_state: PinState,
    ) -> Self {
        Self::from_inner(GpioInner::new_k3(bank, number, gpio), initial_state)
    }

    #[inline]
    pub(super) fn from_inner(inner: GpioInner<'a>, state: PinState) -> Self {
        inner.configure_output(state);
        Self { inner, state }
    }
}

impl ErrorType for Output<'_> {
    type Error = Infallible;
}

impl OutputPin for Output<'_> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.inner.set_state(PinState::Low);
        self.state = PinState::Low;
        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.inner.set_state(PinState::High);
        self.state = PinState::High;
        Ok(())
    }
}

impl StatefulOutputPin for Output<'_> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::High)
    }

    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::Low)
    }
}
