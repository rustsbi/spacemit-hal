use core::marker::PhantomData;

use super::{
    inner::GpioInner,
    register::{k1, k3},
};

/// An externally configured alternate-function pin.
pub struct Function<'a, const B: u8, const N: u8, const F: u8> {
    // Preserve the register borrow and its auto-trait restrictions without
    // retaining register addresses that cannot be used before pinctrl support.
    _inner: PhantomData<GpioInner<'a>>,
}

impl<'a, const B: u8, const N: u8, const F: u8> Function<'a, B, N, F> {
    /// Constructs a K1/M1 alternate-function pin without changing hardware.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k1(gpio: &'a k1::RegisterBlock) -> Self {
        Self::from_inner(GpioInner::new_k1(B, N, gpio))
    }

    /// Constructs a K3 alternate-function pin without changing hardware.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_k3(gpio: &'a k3::RegisterBlock) -> Self {
        Self::from_inner(GpioInner::new_k3(B, N, gpio))
    }

    #[inline]
    fn from_inner(_inner: GpioInner<'a>) -> Self {
        assert!(F < 8, "GPIO alternate function must be in 0..8");
        Self {
            _inner: PhantomData,
        }
    }
}
