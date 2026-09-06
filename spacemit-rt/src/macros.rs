macro_rules! soc {
    (
        $(
            $(#[$doc:meta])*
            pub struct $Ty:ident => $paddr:expr, $DerefTy:ty;
        )+
    ) => {
        $(
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $Ty {
                // Tokens are neither Send nor Sync: MMIO access is not synchronized.
                _private: core::marker::PhantomData<*mut ()>,
            }

            impl $Ty {
                /// Returns the peripheral's physical register address.
                #[inline]
                pub const fn ptr() -> *const $DerefTy {
                    $paddr as *const $DerefTy
                }
            }

            impl core::ops::Deref for $Ty {
                type Target = $DerefTy;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    // SAFETY: Peripherals::steal requires the matching SoC, a live
                    // identity-mapped register block, and exclusive ownership;
                    // the returned reference cannot outlive this token borrow.
                    unsafe { &*Self::ptr() }
                }
            }

            impl core::convert::AsRef<$DerefTy> for $Ty {
                #[inline(always)]
                fn as_ref(&self) -> &$DerefTy {
                    self
                }
            }
        )+
    };
}

macro_rules! impl_uart {
    ($($uart:ident),+ $(,)?) => {
        $(
            impl spacemit_hal::uart::Instance<'static> for $uart {
                fn register_block(self) -> &'static spacemit_hal::uart::RegisterBlock {
                    // SAFETY: acquiring the token requires a permanently valid mapping;
                    // consuming it prevents further access through that token.
                    unsafe { &*Self::ptr() }
                }
            }

            impl<'a> spacemit_hal::uart::Instance<'a> for &'a mut $uart {
                fn register_block(self) -> &'a spacemit_hal::uart::RegisterBlock {
                    self
                }
            }
        )+
    };
}

macro_rules! apbc_clocks {
    (
        $APBC:ident, $RegisterBlock:ty;
        uart { $($uart:ident => $field:ident, $register:ident;)+ }
        i2c { $($i2c:ident => $i2c_field:ident, $i2c_register:ident, $constructor:ident;)+ }
    ) => {
        $(
            // SAFETY: This SoC-specific runtime token identifies one physical UART.
            unsafe impl spacemit_hal::clock::UartId for $uart {}

            // SAFETY: The SoC address map associates this UART with exactly
            // the specified APBC identity; these tokens are consumed, not copied.
            unsafe impl spacemit_hal::uart::ClockedInstance<'static> for $uart {
                type ClockId = $uart;
            }

            // SAFETY: The mutable borrow retains the same identity and keeps
            // the original owner inaccessible while the erased resource exists.
            unsafe impl<'a> spacemit_hal::uart::ClockedInstance<'a> for &'a mut $uart {
                type ClockId = $uart;
            }
        )+

        $(
            // SAFETY: Each SoC-specific token identifies one physical I²C controller.
            unsafe impl spacemit_hal::clock::I2cId for $i2c {}
        )+

        /// Exclusive tokens for the modeled APBC clocks.
        pub struct ApbcClocks<'a> {
            $(
                #[doc = concat!("Exclusive ", stringify!($uart), " clock token.")]
                pub $field: spacemit_hal::clock::UartClock<'a, $uart>,
            )+
            $(
                #[doc = concat!("Exclusive ", stringify!($i2c), " clock token.")]
                pub $i2c_field: spacemit_hal::clock::I2cClock<'a, $i2c>,
            )+
        }

        impl<'a> ApbcClocks<'a> {
            // SAFETY: The caller transfers exclusive, valid APBC access for 'a.
            unsafe fn from_registers(registers: &'a $RegisterBlock) -> Self {
                Self {
                    $(
                        // SAFETY: The SoC map pairs this UART with the specified
                        // register; the caller establishes stable, exclusive access.
                        $field: unsafe {
                            spacemit_hal::clock::UartClock::from_register(
                                &registers.$register, spacemit_hal::clock::Clocks::unknown(),
                            )
                        },
                    )+
                    $(
                        // SAFETY: The SoC map supplies the correct register and
                        // preserves its readable or write-only access policy.
                        $i2c_field: unsafe {
                            spacemit_hal::clock::I2cClock::$constructor(
                                &registers.$i2c_register,
                            )
                        },
                    )+
                }
            }
        }

        impl ApbcClocks<'static> {
            // SAFETY: Peripherals::steal establishes exclusive permanent MMIO access.
            unsafe fn new() -> Self {
                // SAFETY: The caller guarantees the mapped APBC register block;
                // constructing tokens does not read or write its registers.
                unsafe { Self::from_registers(&*$APBC::ptr()) }
            }
        }

        #[cfg(test)]
        mod apbc_clock_tests {
            use super::*;
            use spacemit_hal::{apbc::{TwsiClockReset, UartClockReset}, clock::{Clocks, Hertz}};

            #[test]
            fn uart_tokens_borrow_their_mapped_clock_registers() {
                // SAFETY: The fixture contains only initialized integer MMIO cells.
                let registers: $RegisterBlock = unsafe { core::mem::zeroed() };
                // SAFETY: This test exclusively owns the RAM-backed APBC fixture.
                let mut clocks = unsafe { ApbcClocks::from_registers(&registers) };
                let frequencies = Clocks::new(Some(Hertz(1)), Some(Hertz(2)), None).unwrap();
                $(
                    // SAFETY: Simulated source frequencies and exclusive RAM writes.
                    unsafe {
                        clocks.$field.set_frequencies(frequencies);
                        registers.$register.write(UartClockReset::from_bits(0x13));
                    }
                    assert_eq!(clocks.$field.frequency(), Some(Hertz(2)));
                    // SAFETY: Restore this simulated register before checking the next.
                    unsafe { registers.$register.write(UartClockReset::from_bits(0)) };
                )+
            }

            #[test]
            fn i2c_tokens_preserve_their_mapped_registers_and_readback_policy() {
                // SAFETY: The fixture contains only initialized integer MMIO cells.
                let registers: $RegisterBlock = unsafe { core::mem::zeroed() };
                // SAFETY: This test exclusively owns the RAM-backed APBC fixture.
                let clocks = unsafe { ApbcClocks::from_registers(&registers) };
                $(
                    // SAFETY: Sequential writes to this simulated clock register.
                    unsafe { registers.$i2c_register.write(TwsiClockReset::from_bits(3)) };
                    if stringify!($constructor) == "from_write_only_register" {
                        assert_eq!(clocks.$i2c_field.readback(), None);
                    } else {
                        assert!(clocks.$i2c_field.readback().unwrap().is_enabled());
                    }
                    // SAFETY: Restore the RAM fixture before checking the next token.
                    unsafe { registers.$i2c_register.write(TwsiClockReset::from_bits(0)) };
                )+
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicU32, Ordering};

    static REGISTERS: AtomicU32 = AtomicU32::new(0x1234_5678);

    soc! {
        pub struct Test => &REGISTERS as *const AtomicU32, AtomicU32;
    }

    #[test]
    fn token_borrows_the_register_block() {
        // This test token points to initialized static RAM, not hardware MMIO.
        let token = Test {
            _private: core::marker::PhantomData,
        };
        assert!(core::ptr::eq(Test::ptr(), &REGISTERS));
        assert!(core::ptr::eq(&*token, token.as_ref()));
        assert_eq!(token.load(Ordering::Relaxed), 0x1234_5678);
        assert_eq!(core::mem::size_of::<Test>(), 0);
    }
}
