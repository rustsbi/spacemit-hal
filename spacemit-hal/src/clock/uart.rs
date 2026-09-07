use super::{Clocks, Error, Hertz};
use crate::apbc::UartClockReset;
use core::marker::PhantomData;
use volatile_register::RW;

/// A supported SoC-specific APBC UART identity.
///
/// # Safety
/// Each implementor must identify exactly one physical UART in one SoC;
/// different UART instances must not share the same identity type.
pub unsafe trait UartId {
    /// The matching, permanently mapped APBC UART clock register.
    const CLOCK_REGISTER: *const RW<UartClockReset>;
}

/// A read-only exclusive UART clock token without an assumed frequency.
#[must_use]
pub struct UartClock<I: UartId> {
    _identity: PhantomData<fn() -> I>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<I: UartId> UartClock<I> {
    /// Acquires a UART clock register without accessing hardware.
    ///
    /// # Safety
    /// Own UART I's permanently mapped clock register exclusively; upstream power
    /// must permit access, with no conflicting users or DMA after drop or forget.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new() -> Self {
        Self {
            _identity: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    /// Binds a verified UART frequency while retaining the controller borrows.
    #[inline(always)]
    pub fn with_clock<'b>(
        &'b mut self,
        clocks: &Clocks<'b>,
    ) -> Result<UartFrequency<'b, I>, Error> {
        // SAFETY: The unique token grants access to I's permanent mapping.
        let value = unsafe { &*I::CLOCK_REGISTER }.read();
        if !value.is_enabled() {
            return Err(Error::Disabled);
        }
        let source = value.clock_source().ok_or(Error::ReservedSource)?;
        let frequency = clocks.uart_source(source).ok_or(Error::UnknownFrequency)?;
        Ok(UartFrequency {
            inner: UartFrequencyRef {
                _source: PhantomData,
                frequency,
            },
            _identity: PhantomData,
        })
    }
}

/// A UART input frequency retaining its clock and source borrows.
#[must_use]
pub struct UartFrequency<'a, I: UartId> {
    pub(crate) inner: UartFrequencyRef<'a>,
    _identity: PhantomData<fn() -> I>,
}

impl<I: UartId> UartFrequency<'_, I> {
    /// Returns the verified UART input frequency.
    #[inline]
    pub const fn frequency(&self) -> Hertz {
        self.inner.frequency
    }
}

pub(crate) struct UartFrequencyRef<'a> {
    // Retain the controllers' lifetime, not the address of the Clocks snapshot.
    _source: PhantomData<(&'a mut (), Clocks<'a>)>,
    pub(crate) frequency: Hertz,
}
