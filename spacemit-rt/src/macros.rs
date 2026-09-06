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
