//! GPIO register access types and chip-specific layouts.

use vcell::VolatileCell;

pub(super) mod bank;

/// A read/write-one-to-clear register.
#[repr(transparent)]
pub struct RW1C<T: Copy> {
    register: VolatileCell<T>,
}

impl<T: Copy> RW1C<T> {
    /// Reads the currently asserted status bits.
    #[inline(always)]
    pub fn read(&self) -> T {
        self.register.get()
    }

    /// Clears exactly the status bits selected by `mask`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that acknowledging every selected pending event
    /// is valid for the current peripheral and interrupt state.
    #[inline(always)]
    pub unsafe fn clear(&self, mask: T) {
        self.register.set(mask)
    }
}

// Padding fields are named `_padding_0x...` after their starting byte offset
// relative to the containing struct, including within a nested bank.
pub mod k1;
pub mod k3;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};
    use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};

    use super::{RW1C, bank::BankRegisters, k1, k3};
    use crate::gpio::{Function, Input, Output};

    #[inline]
    unsafe fn read_register<T>(register: *const T) -> u32 {
        // SAFETY: Test register wrappers are repr-transparent over one u32 and
        // `register` points into a live, suitably aligned mock register block.
        unsafe { register.cast::<u32>().read_volatile() }
    }

    #[inline]
    unsafe fn write_register<T>(register: *const T, value: u32) {
        // SAFETY: This simulates a hardware update to an RO/W1C register in a
        // live, suitably aligned mock register block.
        unsafe { register.cast_mut().cast::<u32>().write_volatile(value) };
    }

    #[test]
    fn rw1c_register_layout() {
        assert_eq!(size_of::<RW1C<u8>>(), size_of::<u8>());
        assert_eq!(align_of::<RW1C<u8>>(), align_of::<u8>());
        assert_eq!(size_of::<RW1C<u16>>(), size_of::<u16>());
        assert_eq!(align_of::<RW1C<u16>>(), align_of::<u16>());
        assert_eq!(size_of::<RW1C<u32>>(), 4);
        assert_eq!(align_of::<RW1C<u32>>(), 4);
        assert_eq!(size_of::<RW1C<u64>>(), size_of::<u64>());
        assert_eq!(align_of::<RW1C<u64>>(), align_of::<u64>());
    }

    fn assert_function_preserves_gpio(banks: [BankRegisters<'_>; 4], construct: impl FnOnce()) {
        for bank in banks {
            // SAFETY: These are live mock registers owned by this test.
            unsafe {
                bank.pin_output_set.write(0x1234_5678);
                bank.pin_output_clear.write(0x8765_4321);
                bank.direction_set.write(0xa5a5_a5a5);
                bank.direction_clear.write(0x5a5a_5a5a);
            }
        }

        construct();

        for bank in banks {
            // SAFETY: See `read_register`; mock WO registers retain writes.
            unsafe {
                assert_eq!(read_register(bank.pin_output_set), 0x1234_5678);
                assert_eq!(read_register(bank.pin_output_clear), 0x8765_4321);
                assert_eq!(read_register(bank.direction_set), 0xa5a5_a5a5);
                assert_eq!(read_register(bank.direction_clear), 0x5a5a_5a5a);
            }
        }
    }

    #[test]
    fn k1_function_preserves_gpio_configuration() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k1::RegisterBlock = unsafe { core::mem::zeroed() };
        let banks = core::array::from_fn(|bank| registers.bank(bank as u8).unwrap());
        assert_function_preserves_gpio(banks, || {
            // SAFETY: These uniquely owned mock pins stand in for externally
            // configured pads; no physical pin/function mapping is exercised.
            unsafe {
                let _first = Function::<0, 0, 0>::__new_k1(&registers);
                let _last = Function::<3, 31, 7>::__new_k1(&registers);
            }
        });
    }

    #[test]
    fn k3_function_preserves_gpio_configuration() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k3::RegisterBlock = unsafe { core::mem::zeroed() };
        let banks = core::array::from_fn(|bank| registers.bank(bank as u8).unwrap());
        assert_function_preserves_gpio(banks, || {
            // SAFETY: These uniquely owned mock pins stand in for externally
            // configured pads; no physical pin/function mapping is exercised.
            unsafe {
                let _first = Function::<0, 0, 0>::__new_k3(&registers);
                let _last = Function::<3, 31, 7>::__new_k3(&registers);
            }
        });
    }

    #[test]
    #[should_panic(expected = "GPIO bank 4 must be in 0..4")]
    fn function_rejects_invalid_bank() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k1::RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: The live mock block is valid; checked out-of-range parameters
        // panic before a pin token can be created.
        let _pin = unsafe { Function::<4, 0, 0>::__new_k1(&registers) };
    }

    #[test]
    #[should_panic(expected = "GPIO pin number must be in 0..32")]
    fn function_rejects_invalid_pin() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k3::RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: The live mock block is valid; checked out-of-range parameters
        // panic before a pin token can be created.
        let _pin = unsafe { Function::<0, 32, 0>::__new_k3(&registers) };
    }

    #[test]
    #[should_panic(expected = "GPIO alternate function must be in 0..8")]
    fn function_rejects_invalid_selector() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k1::RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: The live mock block is valid; checked out-of-range parameters
        // panic before a pin token can be created.
        let _pin = unsafe { Function::<0, 0, 8>::__new_k1(&registers) };
    }

    #[test]
    fn k1_input_uses_interleaved_bank() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k1::RegisterBlock = unsafe { core::mem::zeroed() };
        let mask = 1 << 7;
        // SAFETY: This test creates the only logical owner of GPIO2[7].
        let mut input = unsafe { Input::__new_k1(2, 7, &registers) };

        // SAFETY: See `read_register`.
        assert_eq!(
            unsafe { read_register(core::ptr::addr_of!(registers.direction_clear[2])) },
            mask
        );

        // SAFETY: Simulates GPIO_PLR being updated by the external pad.
        unsafe { write_register(core::ptr::addr_of!(registers.pin_level[2]), mask) };
        assert!(input.is_high().unwrap());
    }

    #[test]
    fn k3_output_uses_contiguous_bank_and_tracks_latch_state() {
        // SAFETY: All register wrappers accept the all-zero bit pattern.
        let registers: k3::RegisterBlock = unsafe { core::mem::zeroed() };
        let mask = 1 << 19;
        // SAFETY: This test creates the only logical owner of GPIO3[19].
        let mut output = unsafe { Output::__new_k3(3, 19, &registers, PinState::High) };

        // SAFETY: See `read_register`.
        assert_eq!(
            unsafe { read_register(core::ptr::addr_of!(registers.gpio3.pin_output_set)) },
            mask
        );
        // SAFETY: See `read_register`.
        assert_eq!(
            unsafe { read_register(core::ptr::addr_of!(registers.gpio3.direction_set)) },
            mask
        );
        assert!(output.is_set_high().unwrap());

        output.set_low().unwrap();
        // SAFETY: See `read_register`.
        assert_eq!(
            unsafe { read_register(core::ptr::addr_of!(registers.gpio3.pin_output_clear)) },
            mask
        );
        assert!(output.is_set_low().unwrap());

        let mut input = output.into_input();
        // SAFETY: See `read_register`.
        assert_eq!(
            unsafe { read_register(core::ptr::addr_of!(registers.gpio3.direction_clear)) },
            mask
        );
        // SAFETY: Simulates a high physical level while configured as input.
        unsafe { write_register(core::ptr::addr_of!(registers.gpio3.pin_level), mask) };
        assert!(input.is_high().unwrap());
    }
}
