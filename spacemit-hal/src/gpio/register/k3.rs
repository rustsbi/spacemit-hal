//! K3 GPIO register layout.

use super::{RW1C, bank::BankRegisters};
use volatile_register::{RO, RW, WO};

/// K3 GPIO register block.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO0 registers.
    pub gpio0: Bank,
    _padding_0x03c: [u32; 1],
    /// GPIO1 registers.
    pub gpio1: Bank,
    _padding_0x07c: [u32; 1],
    /// GPIO2 registers.
    pub gpio2: Bank,
    _padding_0x0bc: [u32; 17],
    /// GPIO3 registers.
    pub gpio3: Bank,
}

impl RegisterBlock {
    /// Borrows one GPIO bank.
    #[inline(always)]
    pub(in crate::gpio) const fn bank(&self, bank: u8) -> Option<BankRegisters<'_>> {
        match bank {
            0 => Some(self.gpio0.registers()),
            1 => Some(self.gpio1.registers()),
            2 => Some(self.gpio2.registers()),
            3 => Some(self.gpio3.registers()),
            _ => None,
        }
    }
}

/// Registers for one K3 GPIO bank.
#[repr(C)]
pub struct Bank {
    /// Pin level register (`GPIO_PLR`).
    pub pin_level: RO<u32>,
    /// Pin direction register (`GPIO_PDR`).
    pub pin_direction: RW<u32>,
    /// Atomic pin output-set register (`GPIO_PSR`).
    pub pin_output_set: WO<u32>,
    /// Atomic pin output-clear register (`GPIO_PCR`).
    pub pin_output_clear: WO<u32>,
    /// Rising-edge detect enable register (`GPIO_RER`).
    pub rising_edge_detect_enable: RW<u32>,
    /// Falling-edge detect enable register (`GPIO_FER`).
    pub falling_edge_detect_enable: RW<u32>,
    /// Edge-detect status register (`GPIO_EDR`, read/write-one-to-clear).
    pub edge_detect_status: RW1C<u32>,
    /// Atomic direction-set register (`GPIO_SDR`).
    pub direction_set: WO<u32>,
    /// Atomic direction-clear register (`GPIO_CDR`).
    pub direction_clear: WO<u32>,
    /// Atomic rising-edge-enable-set register (`GPIO_SRER`).
    pub rising_edge_detect_set: WO<u32>,
    /// Atomic rising-edge-enable-clear register (`GPIO_CRER`).
    pub rising_edge_detect_clear: WO<u32>,
    /// Atomic falling-edge-enable-set register (`GPIO_SFER`).
    pub falling_edge_detect_set: WO<u32>,
    /// Atomic falling-edge-enable-clear register (`GPIO_CFER`).
    pub falling_edge_detect_clear: WO<u32>,
    /// Application-processor interrupt mask register (`GPIO_APMASK`).
    pub ap_interrupt_mask: RW<u32>,
    /// Companion-processor interrupt mask register (`GPIO_CPMASK`).
    pub cp_interrupt_mask: RW<u32>,
}

impl Bank {
    #[inline(always)]
    const fn registers(&self) -> BankRegisters<'_> {
        // SAFETY: This complete, borrowed K3 bank has 4-byte register spacing.
        unsafe { BankRegisters::new(core::ptr::NonNull::from_ref(self).cast(), 4) }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bank, RegisterBlock};
    use crate::{
        gpio::{FlexPad, PadExt, PinState},
        mfpr,
        prelude::*,
    };
    use core::mem::{align_of, offset_of, size_of};
    use embedded_hal::digital::StatefulOutputPin;

    fn fixture() -> (RegisterBlock, mfpr::k3::RegisterBlock) {
        // SAFETY: Both blocks contain only zero-valid integer register cells.
        unsafe { core::mem::zeroed() }
    }

    unsafe fn read(register: &volatile_register::WO<u32>) -> u32 {
        // SAFETY: Callers supply live RAM fixtures, never real write-only MMIO.
        unsafe { core::ptr::from_ref(register).cast::<u32>().read_volatile() }
    }

    #[test]
    fn bank_layout() {
        assert_eq!(offset_of!(Bank, pin_level), 0x00);
        assert_eq!(offset_of!(Bank, pin_direction), 0x04);
        assert_eq!(offset_of!(Bank, pin_output_set), 0x08);
        assert_eq!(offset_of!(Bank, pin_output_clear), 0x0c);
        assert_eq!(offset_of!(Bank, rising_edge_detect_enable), 0x10);
        assert_eq!(offset_of!(Bank, falling_edge_detect_enable), 0x14);
        assert_eq!(offset_of!(Bank, edge_detect_status), 0x18);
        assert_eq!(offset_of!(Bank, direction_set), 0x1c);
        assert_eq!(offset_of!(Bank, direction_clear), 0x20);
        assert_eq!(offset_of!(Bank, rising_edge_detect_set), 0x24);
        assert_eq!(offset_of!(Bank, rising_edge_detect_clear), 0x28);
        assert_eq!(offset_of!(Bank, falling_edge_detect_set), 0x2c);
        assert_eq!(offset_of!(Bank, falling_edge_detect_clear), 0x30);
        assert_eq!(offset_of!(Bank, ap_interrupt_mask), 0x34);
        assert_eq!(offset_of!(Bank, cp_interrupt_mask), 0x38);
        assert_eq!(size_of::<Bank>(), 0x3c);
        assert_eq!(align_of::<Bank>(), 4);
    }

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, gpio0), 0x00);
        assert_eq!(offset_of!(RegisterBlock, gpio1), 0x40);
        assert_eq!(offset_of!(RegisterBlock, gpio2), 0x80);
        assert_eq!(offset_of!(RegisterBlock, gpio3), 0x100);
        assert_eq!(size_of::<RegisterBlock>(), 0x13c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn bank_mapping() {
        let (gpio, _) = fixture();
        let base = &gpio as *const _ as usize;
        for (i, offset) in [0, 0x40, 0x80, 0x100].into_iter().enumerate() {
            let bank = gpio.bank(i as u8).unwrap();
            assert_eq!(bank.pin_level() as *const _ as usize - base, offset);
            assert_eq!(
                bank.pin_output_set() as *const _ as usize - base,
                offset + 8
            );
            assert_eq!(
                bank.pin_output_clear() as *const _ as usize - base,
                offset + 12
            );
            assert_eq!(
                bank.direction_clear() as *const _ as usize - base,
                offset + 32
            );
            assert_eq!(
                bank.direction_set() as *const _ as usize - base,
                offset + 0x1c
            );
        }
        assert!(gpio.bank(4).is_none());
    }

    #[test]
    fn pad_modes() {
        let (gpio, mfpr) = fixture();
        for n in 0..128 {
            let bank = gpio.bank(n / 32).unwrap();
            let config = &mfpr.gpio[usize::from(n)];
            let aliases = [
                bank.pin_output_set(),
                bank.pin_output_clear(),
                bank.direction_set(),
                bank.direction_clear(),
            ];
            let mask = 1 << (n % 32);
            // SAFETY: Exclusive, initialized RAM fixtures; WO cells retain writes.
            unsafe {
                config.write(0xffff_fff8);
                for reg in aliases {
                    reg.write(0);
                }
                let pad = FlexPad::__new_k3(n, &gpio, &mfpr).into_function::<7>();
                assert_eq!(config.read(), 0xffff_ffff);
                assert_eq!(aliases.map(|reg| read(reg)), [0; 4]);
                let pad = FlexPad::from(pad).into_function::<0>();
                assert_eq!(config.read(), 0xffff_fff8);

                let mut output = FlexPad::from(pad).into_output(PinState::High);
                assert_eq!(config.read(), 0xffff_fff8);
                assert_eq!(
                    (read(bank.pin_output_set()), read(bank.direction_set())),
                    (mask, mask)
                );
                assert!(output.is_set_high().unwrap());
                output.set_low().unwrap();
                assert_eq!(read(bank.pin_output_clear()), mask);
                assert!(output.is_set_low().unwrap());

                let mut input = output.into_input();
                assert_eq!(read(bank.direction_clear()), mask);
                core::ptr::from_ref(bank.pin_level())
                    .cast_mut()
                    .cast::<u32>()
                    .write_volatile(mask);
                assert!(input.is_high().unwrap());
            }
        }
    }

    #[test]
    #[should_panic(expected = "GPIO number must be in 0..128")]
    fn invalid_pin() {
        let (gpio, mfpr) = fixture();
        // SAFETY: Valid RAM fixtures; the number is checked before indexing.
        let _ = unsafe { FlexPad::__new_k3(128, &gpio, &mfpr) };
    }

    #[test]
    fn temporary_modes_restore_after_panic() {
        extern crate std;
        use std::panic::{AssertUnwindSafe, catch_unwind};

        let (gpio, mfpr) = fixture();
        let bank = gpio.bank(0).unwrap();
        // SAFETY: Exclusive RAM fixtures, including simulated WO alias readback.
        unsafe {
            let mut input = FlexPad::__new_k3(0, &gpio, &mfpr).into_input();
            let result = catch_unwind(AssertUnwindSafe(|| {
                input.with_output(PinState::High, |_| {
                    bank.direction_clear().write(0);
                    panic!();
                });
            }));
            assert!(result.is_err());
            assert_eq!(read(bank.direction_clear()), 1);

            let mut output = input.into_output(PinState::High);
            let result = catch_unwind(AssertUnwindSafe(|| {
                output.with_input(|_| {
                    bank.pin_output_set().write(0);
                    bank.direction_set().write(0);
                    panic!();
                });
            }));
            assert!(result.is_err());
            assert_eq!(
                (read(bank.pin_output_set()), read(bank.direction_set())),
                (1, 1)
            );
            assert!(output.is_set_high().unwrap());
        }
    }
}
