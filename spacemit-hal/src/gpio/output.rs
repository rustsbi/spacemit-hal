use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

use super::{
    inner::GpioInner,
    input::Input,
    register::{k1, k3},
};

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

    /// Borrows the pin for temporary use as a digital input.
    #[inline]
    pub fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a>) -> T,
    {
        let mut input = Input::from_inner(self.inner);
        let result = f(&mut input);
        self.inner.configure_output(self.state);
        result
    }

    /// Constructs a K1/M1 output pin.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k1(
        bank: u8,
        number: u8,
        gpio: &'a k1::RegisterBlock,
        initial_state: PinState,
    ) -> Self {
        Self::new(GpioInner::new_k1(bank, number, gpio), initial_state)
    }

    /// Constructs a K3 output pin.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k3(
        bank: u8,
        number: u8,
        gpio: &'a k3::RegisterBlock,
        initial_state: PinState,
    ) -> Self {
        Self::new(GpioInner::new_k3(bank, number, gpio), initial_state)
    }

    #[inline]
    fn new(pin: GpioInner<'a>, initial_state: PinState) -> Self {
        pin.configure_output(initial_state);
        Self::from_inner(pin, initial_state)
    }

    #[inline]
    pub(super) const fn from_inner(inner: GpioInner<'a>, state: PinState) -> Self {
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
