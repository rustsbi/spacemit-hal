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

macro_rules! impl_qspi {
    ($peripheral:ident) => {
        // SAFETY: Peripheral acquisition transfers this exclusive, fixed register mapping.
        unsafe impl<'a> spacemit_hal::qspi::Instance<'a> for $peripheral {
            fn register_block(self) -> &'a spacemit_hal::qspi::RegisterBlock {
                // SAFETY: Consuming the token transfers its permanently valid mapping.
                unsafe { &*Self::ptr() }
            }
        }

        // SAFETY: The borrow retains the controller's exclusive mapping for its lifetime.
        unsafe impl<'a> spacemit_hal::qspi::Instance<'a> for &'a mut $peripheral {
            fn register_block(self) -> &'a spacemit_hal::qspi::RegisterBlock {
                self
            }
        }
    };
}

macro_rules! impl_clock_controller {
    ($module:ident, $peripheral:ident, $register:ty) => {
        // SAFETY: Peripherals acquisition establishes exclusive, permanently
        // mapped controller access and the shared-source/consumer contract.
        unsafe impl<'a> spacemit_hal::$module::Instance<'a> for $peripheral {
            type RegisterBlock = $register;

            #[inline]
            fn register_block(self) -> &'a Self::RegisterBlock {
                // SAFETY: Consuming the token transfers its permanent mapping.
                unsafe { &*Self::ptr() }
            }
        }

        // SAFETY: The mutable borrow retains the token's exclusive access.
        unsafe impl<'a> spacemit_hal::$module::Instance<'a> for &'a mut $peripheral {
            type RegisterBlock = $register;

            #[inline]
            fn register_block(self) -> &'a Self::RegisterBlock {
                self
            }
        }
    };
}

macro_rules! impl_uart {
    ($($uart:ident),+ $(,)?) => {
        $(
            impl<'a> spacemit_hal::uart::Instance<'a> for $uart {
                #[inline]
                fn register_block(self) -> &'a spacemit_hal::uart::RegisterBlock {
                    // SAFETY: acquiring the token requires a permanently valid mapping;
                    // consuming it prevents further access through that token.
                    unsafe { &*Self::ptr() }
                }
            }

            impl<'a> spacemit_hal::uart::Instance<'a> for &'a mut $uart {
                #[inline]
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
            #[inline]
            fn into_flex<'a>(self) -> spacemit_hal::gpio::FlexPad<'a> {
                // SAFETY: Consuming this unique token transfers permanent access
                // to GPIO N and its MFPR, established by Peripherals::steal.
                unsafe {
                    spacemit_hal::gpio::FlexPad::$constructor(N, &*GPIO::ptr(), &*MFPR::ptr())
                }
            }

            #[inline]
            fn as_flex(&mut self) -> spacemit_hal::gpio::FlexPad<'_> {
                // SAFETY: The pad token guarantees valid mappings; this resource
                // retains the mutable borrow and cannot outlive it.
                unsafe {
                    spacemit_hal::gpio::FlexPad::$constructor(N, &*GPIO::ptr(), &*MFPR::ptr())
                }
            }
        }

        impl<'a, const N: u8> spacemit_hal::gpio::PadExt<'a> for Pad<N> {
            #[inline]
            fn into_input(self) -> spacemit_hal::gpio::Input<'a> {
                self.into_flex().into_input()
            }

            #[inline]
            fn into_output(self, initial: spacemit_hal::gpio::PinState) -> spacemit_hal::gpio::Output<'a> {
                self.into_flex().into_output(initial)
            }

            #[inline]
            fn into_function<const F: u8>(self) -> spacemit_hal::gpio::Function<'a, F> {
                self.into_flex().into_function::<F>()
            }
        }

        impl<'a, const N: u8> spacemit_hal::gpio::PadExt<'a> for &'a mut Pad<N> {
            #[inline]
            fn into_input(self) -> spacemit_hal::gpio::Input<'a> {
                self.as_flex().into_input()
            }

            #[inline]
            fn into_output(self, initial: spacemit_hal::gpio::PinState) -> spacemit_hal::gpio::Output<'a> {
                self.as_flex().into_output(initial)
            }

            #[inline]
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
            #[inline]
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

    };
}

macro_rules! impl_i2c_pads {
    ($i2c:ident, $scl:literal, $sda:literal, $configuration:literal) => {
        impl_i2c_pads!(@pad $scl, $configuration);
        impl_i2c_pads!(@pad $sda, $configuration);
        // SAFETY: The SoC mux table maps this pair to the stated I²C controller.
        unsafe impl<'a> spacemit_hal::i2c::IntoPads<'a, $i2c> for (Pad<$scl>, Pad<$sda>) {
            #[inline]
            fn into_i2c_pads(mut self) -> spacemit_hal::i2c::Pads<'a> {
                self.0.configure_i2c();
                self.1.configure_i2c();
                // SAFETY: Consuming both unique tokens retains their configured route for 'a.
                unsafe { spacemit_hal::i2c::Pads::__configured() }
            }
        }
        // SAFETY: The same route retains both mutable pad borrows.
        unsafe impl<'a> spacemit_hal::i2c::IntoPads<'a, $i2c>
            for (&'a mut Pad<$scl>, &'a mut Pad<$sda>)
        {
            #[inline]
            fn into_i2c_pads(self) -> spacemit_hal::i2c::Pads<'a> {
                self.0.configure_i2c();
                self.1.configure_i2c();
                // SAFETY: The returned proof retains both exclusive pad borrows for 'a.
                unsafe { spacemit_hal::i2c::Pads::__configured() }
            }
        }
    };
    (@pad $pin:literal, $configuration:literal) => {
        impl Pad<$pin> {
            #[inline]
            fn configure_i2c(&mut self) {
                // SAFETY: This token exclusively owns the mapped pad; the SoC supplies its configuration.
                unsafe {
                    let mfpr = &*MFPR::ptr();
                    mfpr.gpio[$pin].write($configuration);
                }
                riscv::asm::fence();
            }
        }

        impl<'a> spacemit_hal::i2c::IntoI2c<'a> for Pad<$pin> {
            #[inline]
            fn into_i2c(mut self) -> spacemit_hal::gpio::FlexPad<'a> {
                self.configure_i2c();
                self.into_flex()
            }
        }

        impl<'a> spacemit_hal::i2c::IntoI2c<'a> for &'a mut Pad<$pin> {
            #[inline]
            fn into_i2c(self) -> spacemit_hal::gpio::FlexPad<'a> {
                self.configure_i2c();
                self.as_flex()
            }
        }
    };
}

macro_rules! apbc_clocks {
    (
        $APBC:ident, $RegisterBlock:ty;
        uart { $($uart:ident => $field:ident, $register:ident;)+ }
        i2c { $($i2c:ident => $i2c_field:ident, $i2c_register:ident;)+ }
        $(counter { $counter:ident => $counter_field:ident, $counter_register:ident; })?
    ) => {
        $(
            // SAFETY: This SoC-specific runtime token identifies one physical UART.
            unsafe impl spacemit_hal::clock::UartId for $uart {
                const CLOCK_REGISTER: *const volatile_register::RW<spacemit_hal::apbc::UartClockReset> =
                    $APBC::ptr().cast::<u8>()
                        .wrapping_add(core::mem::offset_of!($RegisterBlock, $register)).cast();
            }

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
            unsafe impl spacemit_hal::clock::I2cId for $i2c {
                const CLOCK_REGISTER: *const volatile_register::WO<spacemit_hal::apbc::TwsiClockReset> =
                    // Keep only write capability, including K1 TWSI8's WO register.
                    $APBC::ptr().cast::<u8>()
                        .wrapping_add(core::mem::offset_of!($RegisterBlock, $i2c_register)).cast();
            }

            // SAFETY: Peripherals::steal grants permanent, exclusive MMIO and clock access.
            unsafe impl<'a> spacemit_hal::i2c::Instance<'a> for $i2c {
                type ClockId = $i2c;
                #[inline]
                fn register_block(self) -> &'a spacemit_hal::i2c::RegisterBlock {
                    // SAFETY: Consuming the token transfers its permanent mapping.
                    unsafe { &*Self::ptr() }
                }
            }

            // SAFETY: The mutable borrow retains the token's platform guarantees.
            unsafe impl<'a> spacemit_hal::i2c::Instance<'a> for &'a mut $i2c {
                type ClockId = $i2c;
                #[inline]
                fn register_block(self) -> &'a spacemit_hal::i2c::RegisterBlock { self }
            }
        )+

        $(
            // SAFETY: This SoC identity names exactly this generic-counter selector.
            unsafe impl spacemit_hal::clock::CounterId for $counter {
                const CLOCK_REGISTER: *const volatile_register::RW<spacemit_hal::apbc::k1::CounterClockControl> =
                    $APBC::ptr().cast::<u8>()
                        .wrapping_add(core::mem::offset_of!($RegisterBlock, $counter_register)).cast();
            }
        )?

        /// Exclusive zero-sized tokens for the modeled APBC clocks.
        pub struct ApbcClocks {
            $(
                /// Exclusive generic-counter clock-source token.
                pub $counter_field: spacemit_hal::clock::CounterClock<$counter>,
            )?
            $(
                #[doc = concat!("Exclusive ", stringify!($uart), " clock token.")]
                pub $field: spacemit_hal::clock::UartClock<$uart>,
            )+
            $(
                #[doc = concat!("Exclusive ", stringify!($i2c), " clock token.")]
                pub $i2c_field: spacemit_hal::clock::I2cClock<$i2c>,
            )+
        }

        impl ApbcClocks {
            // SAFETY: Peripherals::steal transfers exclusive permanent APBC access.
            #[inline(always)]
            const unsafe fn new() -> Self {
                Self {
                    $(
                        // SAFETY: This disjoint K1 counter clock register is
                        // exclusively transferred with the other APBC fields.
                        $counter_field: unsafe {
                            spacemit_hal::clock::CounterClock::__new()
                        },
                    )?
                    $(
                        // SAFETY: The SoC map pairs this UART with the specified
                        // register; the caller establishes stable, exclusive access.
                        $field: unsafe {
                            spacemit_hal::clock::UartClock::__new()
                        },
                    )+
                    $(
                        // SAFETY: The SoC map supplies the correct register and
                        // preserves its readable or write-only access policy.
                        $i2c_field: unsafe {
                            spacemit_hal::clock::I2cClock::__new()
                        },
                    )+
                }
            }
        }

        #[cfg(test)]
        #[test]
        fn apbc_clock_tokens_are_zero_sized_and_correctly_mapped() {
            use core::mem::{offset_of, size_of};
            use spacemit_hal::clock;
            $(
                assert_eq!(size_of::<clock::UartClock<$uart>>(), 0);
                assert_eq!(
                    <$uart as clock::UartId>::CLOCK_REGISTER as usize,
                    $APBC::ptr() as usize + offset_of!($RegisterBlock, $register),
                );
            )+
            $(
                assert_eq!(size_of::<clock::I2cClock<$i2c>>(), 0);
                assert_eq!(
                    <$i2c as clock::I2cId>::CLOCK_REGISTER as usize,
                    $APBC::ptr() as usize + offset_of!($RegisterBlock, $i2c_register),
                );
            )+
            $(
                assert_eq!(size_of::<clock::CounterClock<$counter>>(), 0);
                assert_eq!(
                    <$counter as clock::CounterId>::CLOCK_REGISTER as usize,
                    $APBC::ptr() as usize + offset_of!($RegisterBlock, $counter_register),
                );
            )?
        }

    };
}
