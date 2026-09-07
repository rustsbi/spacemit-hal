use super::{Function, Input, Output, PadExt, inner::GpioInner, k1, k3};
use crate::mfpr;
use embedded_hal::digital::PinState;

/// An exclusive pad with erased pin identity.
#[must_use]
pub struct FlexPad<'a> {
    pub(super) inner: GpioInner<'a>,
}

impl<'a> FlexPad<'a> {
    /// Acquires one K1/M1 pad without changing its configuration.
    ///
    /// # Safety
    /// Transfer exclusive access to this GPIO bit and its MFPR register for 'a;
    /// both mappings, power and clocks must remain valid, and no other code,
    /// hart, peripheral or DMA may control this pad during that lifetime.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k1(
        number: u8,
        gpio: &'a k1::RegisterBlock,
        mfpr: &'a mfpr::k1::RegisterBlock,
    ) -> Self {
        assert!(number < 128, "GPIO number must be in 0..128");
        // Linux pinctrl-k1.c: k1_pin_data[].gpiofunc.
        let function = match number {
            70..=73 | 93..=103 => 1,
            104..=109 => 4,
            _ => 0,
        };
        Self {
            inner: GpioInner::new_k1(number / 32, number % 32, gpio)
                .with_configuration(&mfpr.gpio[usize::from(number)], function),
        }
    }

    /// Acquires one K3 GPIO pad without changing its configuration.
    ///
    /// # Safety
    /// Transfer exclusive access to this GPIO bit and its MFPR register for 'a;
    /// both mappings, power and clocks must remain valid, and no other code,
    /// hart, peripheral or DMA may control this pad during that lifetime.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k3(
        number: u8,
        gpio: &'a k3::RegisterBlock,
        mfpr: &'a mfpr::k3::RegisterBlock,
    ) -> Self {
        assert!(number < 128, "GPIO number must be in 0..128");
        Self {
            inner: GpioInner::new_k3(number / 32, number % 32, gpio)
                .with_configuration(&mfpr.gpio[usize::from(number)], 0),
        }
    }
}

impl<'a> PadExt<'a> for FlexPad<'a> {
    #[inline]
    fn into_input(self) -> Input<'a> {
        Input::from_inner(self.inner)
    }

    #[inline]
    fn into_output(self, initial_state: PinState) -> Output<'a> {
        Output::from_inner(self.inner, initial_state)
    }

    #[inline]
    fn into_function<const F: u8>(self) -> Function<'a, F> {
        self.inner.configure_function(F);
        Function { pad: self }
    }
}
