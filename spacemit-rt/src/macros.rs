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
            impl<'a> spacemit_hal::uart::Instance<'a> for $uart {
                fn register_block(self) -> &'a spacemit_hal::uart::RegisterBlock {
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

macro_rules! gpio_pads {
    ($constructor:ident) => {
        gpio_pads! {
            @pads $constructor;
            gpio0: 0; gpio1: 1; gpio2: 2; gpio3: 3;
            gpio4: 4; gpio5: 5; gpio6: 6; gpio7: 7;
            gpio8: 8; gpio9: 9; gpio10: 10; gpio11: 11;
            gpio12: 12; gpio13: 13; gpio14: 14; gpio15: 15;
            gpio16: 16; gpio17: 17; gpio18: 18; gpio19: 19;
            gpio20: 20; gpio21: 21; gpio22: 22; gpio23: 23;
            gpio24: 24; gpio25: 25; gpio26: 26; gpio27: 27;
            gpio28: 28; gpio29: 29; gpio30: 30; gpio31: 31;
            gpio32: 32; gpio33: 33; gpio34: 34; gpio35: 35;
            gpio36: 36; gpio37: 37; gpio38: 38; gpio39: 39;
            gpio40: 40; gpio41: 41; gpio42: 42; gpio43: 43;
            gpio44: 44; gpio45: 45; gpio46: 46; gpio47: 47;
            gpio48: 48; gpio49: 49; gpio50: 50; gpio51: 51;
            gpio52: 52; gpio53: 53; gpio54: 54; gpio55: 55;
            gpio56: 56; gpio57: 57; gpio58: 58; gpio59: 59;
            gpio60: 60; gpio61: 61; gpio62: 62; gpio63: 63;
            gpio64: 64; gpio65: 65; gpio66: 66; gpio67: 67;
            gpio68: 68; gpio69: 69; gpio70: 70; gpio71: 71;
            gpio72: 72; gpio73: 73; gpio74: 74; gpio75: 75;
            gpio76: 76; gpio77: 77; gpio78: 78; gpio79: 79;
            gpio80: 80; gpio81: 81; gpio82: 82; gpio83: 83;
            gpio84: 84; gpio85: 85; gpio86: 86; gpio87: 87;
            gpio88: 88; gpio89: 89; gpio90: 90; gpio91: 91;
            gpio92: 92; gpio93: 93; gpio94: 94; gpio95: 95;
            gpio96: 96; gpio97: 97; gpio98: 98; gpio99: 99;
            gpio100: 100; gpio101: 101; gpio102: 102; gpio103: 103;
            gpio104: 104; gpio105: 105; gpio106: 106; gpio107: 107;
            gpio108: 108; gpio109: 109; gpio110: 110; gpio111: 111;
            gpio112: 112; gpio113: 113; gpio114: 114; gpio115: 115;
            gpio116: 116; gpio117: 117; gpio118: 118; gpio119: 119;
            gpio120: 120; gpio121: 121; gpio122: 122; gpio123: 123;
            gpio124: 124; gpio125: 125; gpio126: 126; gpio127: 127;
        }
    };
    (@pads $constructor:ident; $($field:ident: $number:literal;)+) => {
        /// An exclusive GPIO bit and its SoC-specific MFPR configuration.
        pub struct Pad<const N: u8> {
            _private: core::marker::PhantomData<*mut ()>,
        }

        impl<const N: u8> Pad<N> {
            fn into_flex<'a>(self) -> spacemit_hal::gpio::FlexPad<'a> {
                // SAFETY: Consuming this unique token transfers permanent access
                // to GPIO N and its MFPR, established by Peripherals::steal.
                unsafe {
                    spacemit_hal::gpio::FlexPad::$constructor(N, &*GPIO::ptr(), &*MFPR::ptr())
                }
            }

            fn as_flex(&mut self) -> spacemit_hal::gpio::FlexPad<'_> {
                // SAFETY: The pad token guarantees valid mappings; this resource
                // retains the mutable borrow and cannot outlive it.
                unsafe {
                    spacemit_hal::gpio::FlexPad::$constructor(N, &*GPIO::ptr(), &*MFPR::ptr())
                }
            }
        }

        impl<'a, const N: u8> spacemit_hal::gpio::PadExt<'a> for Pad<N> {
            fn into_input(self) -> spacemit_hal::gpio::Input<'a> {
                self.into_flex().into_input()
            }

            fn into_output(self, initial: spacemit_hal::gpio::PinState) -> spacemit_hal::gpio::Output<'a> {
                self.into_flex().into_output(initial)
            }

            fn into_function<const F: u8>(self) -> spacemit_hal::gpio::Function<'a, F> {
                self.into_flex().into_function::<F>()
            }
        }

        impl<'a, const N: u8> spacemit_hal::gpio::PadExt<'a> for &'a mut Pad<N> {
            fn into_input(self) -> spacemit_hal::gpio::Input<'a> {
                self.as_flex().into_input()
            }

            fn into_output(self, initial: spacemit_hal::gpio::PinState) -> spacemit_hal::gpio::Output<'a> {
                self.as_flex().into_output(initial)
            }

            fn into_function<const F: u8>(self) -> spacemit_hal::gpio::Function<'a, F> {
                self.as_flex().into_function::<F>()
            }
        }

        /// Exclusive GPIO0 through GPIO127 pad tokens.
        pub struct GpioPads {
            $(
                #[doc = concat!("GPIO", stringify!($number), " pad.")]
                pub $field: Pad<$number>,
            )+
        }

        impl GpioPads {
            // SAFETY: The caller transfers all GPIO bits and MFPR registers once.
            unsafe fn new() -> Self {
                Self {
                    $($field: Pad { _private: core::marker::PhantomData },)+
                }
            }
        }
    };
}

macro_rules! impl_uart_pads {
    ($(($number:literal, $function:literal): $Trait:ident, $method:ident, $uart:ident;)+) => {
        $(
            // SAFETY: The cited SoC mux table routes this exact pad to this UART.
            unsafe impl<'a> spacemit_hal::uart::$Trait<'a, $uart> for Pad<$number> {
                #[inline]
                fn $method(self) -> spacemit_hal::gpio::FlexPad<'a> {
                    use spacemit_hal::gpio::PadExt;
                    self.into_function::<$function>().into()
                }
            }

            // SAFETY: The route is unchanged and erasure retains the mutable borrow.
            unsafe impl<'a> spacemit_hal::uart::$Trait<'a, $uart> for &'a mut Pad<$number> {
                #[inline]
                fn $method(self) -> spacemit_hal::gpio::FlexPad<'a> {
                    use spacemit_hal::gpio::PadExt;
                    self.into_function::<$function>().into()
                }
            }
        )+

        #[cfg(test)]
        mod uart_pad_tests {
            use super::*;

            #[test]
            fn routes_accept_owned_and_borrowed_pads() {
                $(
                    {
                        fn route<'a, T: spacemit_hal::uart::$Trait<'a, $uart>>() {}
                        route::<Pad<$number>>();
                        route::<&mut Pad<$number>>();
                    }
                )+
                assert_eq!(core::mem::size_of::<GpioPads>(), 0);
                assert_eq!(core::mem::size_of::<Pad<0>>(), 0);
            }
        }
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

            // SAFETY: The SoC map matches this UART to its clock; Peripherals::steal
            // guarantees permanent valid access, stable power/clocks and no conflicting users or DMA.
            unsafe impl<'a> spacemit_hal::uart::ClockedInstance<'a> for $uart {
                type ClockId = $uart;
            }

            // SAFETY: Same platform guarantees as the owned token; the borrow excludes its owner.
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
            // SAFETY: Transfer exclusive APBC access and stable upstream power/clocks
            // for 'a, permitting UART access when enabled without external interference.
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
