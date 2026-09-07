//! K1/M1 GPIO register layout.

use super::{RW1C, bank::BankRegisters};
use volatile_register::{RO, RW, WO};

/// K1/M1 GPIO register block.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO0..2 pin level registers (`GPIO_PLR`).
    pub pin_level: [RO<u32>; 3],
    /// GPIO0..2 pin direction registers (`GPIO_PDR`).
    pub pin_direction: [RW<u32>; 3],
    /// GPIO0..2 atomic pin output-set registers (`GPIO_PSR`).
    pub pin_output_set: [WO<u32>; 3],
    /// GPIO0..2 atomic pin output-clear registers (`GPIO_PCR`).
    pub pin_output_clear: [WO<u32>; 3],
    /// GPIO0..2 rising-edge detect enable registers (`GPIO_RER`).
    pub rising_edge_detect_enable: [RW<u32>; 3],
    /// GPIO0..2 falling-edge detect enable registers (`GPIO_FER`).
    pub falling_edge_detect_enable: [RW<u32>; 3],
    /// GPIO0..2 edge-detect status registers (`GPIO_EDR`, read/write-one-to-clear).
    pub edge_detect_status: [RW1C<u32>; 3],
    /// GPIO0..2 atomic direction-set registers (`GPIO_SDR`).
    pub direction_set: [WO<u32>; 3],
    /// GPIO0..2 atomic direction-clear registers (`GPIO_CDR`).
    pub direction_clear: [WO<u32>; 3],
    /// GPIO0..2 atomic rising-edge-enable-set registers (`GPIO_SRER`).
    pub rising_edge_detect_set: [WO<u32>; 3],
    /// GPIO0..2 atomic rising-edge-enable-clear registers (`GPIO_CRER`).
    pub rising_edge_detect_clear: [WO<u32>; 3],
    /// GPIO0..2 atomic falling-edge-enable-set registers (`GPIO_SFER`).
    pub falling_edge_detect_set: [WO<u32>; 3],
    /// GPIO0..2 atomic falling-edge-enable-clear registers (`GPIO_CFER`).
    pub falling_edge_detect_clear: [WO<u32>; 3],
    /// GPIO0..2 application-processor interrupt mask registers (`GPIO_APMASK`).
    pub ap_interrupt_mask: [RW<u32>; 3],
    /// GPIO0..2 companion-processor interrupt mask registers (`GPIO_CPMASK`).
    pub cp_interrupt_mask: [RW<u32>; 3],
    _padding_0x0b4: [u32; 19],
    /// GPIO3 registers.
    pub gpio3: Bank,
}

impl RegisterBlock {
    #[inline(always)]
    const fn interleaved_bank(&self, index: usize) -> BankRegisters<'_> {
        // SAFETY: bank() selects 0..3; the whole block retains pointer provenance
        // across the interleaved arrays, whose register kinds are 12 bytes apart.
        unsafe {
            BankRegisters::new(
                core::ptr::NonNull::from_ref(self)
                    .cast::<u8>()
                    .add(index * 4),
                12,
            )
        }
    }

    /// Borrows one GPIO bank.
    #[inline(always)]
    pub(in crate::gpio) const fn bank(&self, bank: u8) -> Option<BankRegisters<'_>> {
        match bank {
            0..=2 => Some(self.interleaved_bank(bank as usize)),
            3 => Some(self.gpio3.registers()),
            _ => None,
        }
    }
}

/// Register group for K1/M1 GPIO3.
#[repr(C)]
pub struct Bank {
    /// Pin level register (`GPIO_PLR`).
    pub pin_level: RO<u32>,
    _padding_0x004: [u32; 2],
    /// Pin direction register (`GPIO_PDR`).
    pub pin_direction: RW<u32>,
    _padding_0x010: [u32; 2],
    /// Atomic pin output-set register (`GPIO_PSR`).
    pub pin_output_set: WO<u32>,
    _padding_0x01c: [u32; 2],
    /// Atomic pin output-clear register (`GPIO_PCR`).
    pub pin_output_clear: WO<u32>,
    _padding_0x028: [u32; 2],
    /// Rising-edge detect enable register (`GPIO_RER`).
    pub rising_edge_detect_enable: RW<u32>,
    _padding_0x034: [u32; 2],
    /// Falling-edge detect enable register (`GPIO_FER`).
    pub falling_edge_detect_enable: RW<u32>,
    _padding_0x040: [u32; 2],
    /// Edge-detect status register (`GPIO_EDR`, read/write-one-to-clear).
    pub edge_detect_status: RW1C<u32>,
    _padding_0x04c: [u32; 2],
    /// Atomic direction-set register (`GPIO_SDR`).
    pub direction_set: WO<u32>,
    _padding_0x058: [u32; 2],
    /// Atomic direction-clear register (`GPIO_CDR`).
    pub direction_clear: WO<u32>,
    _padding_0x064: [u32; 2],
    /// Atomic rising-edge-enable-set register (`GPIO_SRER`).
    pub rising_edge_detect_set: WO<u32>,
    _padding_0x070: [u32; 2],
    /// Atomic rising-edge-enable-clear register (`GPIO_CRER`).
    pub rising_edge_detect_clear: WO<u32>,
    _padding_0x07c: [u32; 2],
    /// Atomic falling-edge-enable-set register (`GPIO_SFER`).
    pub falling_edge_detect_set: WO<u32>,
    _padding_0x088: [u32; 2],
    /// Atomic falling-edge-enable-clear register (`GPIO_CFER`).
    pub falling_edge_detect_clear: WO<u32>,
    _padding_0x094: [u32; 2],
    /// Application-processor interrupt mask register (`GPIO_APMASK`).
    pub ap_interrupt_mask: RW<u32>,
    _padding_0x0a0: [u32; 2],
    /// Companion-processor interrupt mask register (`GPIO_CPMASK`).
    pub cp_interrupt_mask: RW<u32>,
}

impl Bank {
    #[inline(always)]
    const fn registers(&self) -> BankRegisters<'_> {
        // SAFETY: This complete, borrowed K1 bank has 12-byte register spacing.
        unsafe { BankRegisters::new(core::ptr::NonNull::from_ref(self).cast(), 12) }
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

    fn fixture() -> (RegisterBlock, mfpr::k1::RegisterBlock) {
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
        assert_eq!(offset_of!(Bank, pin_direction), 0x0c);
        assert_eq!(offset_of!(Bank, pin_output_set), 0x18);
        assert_eq!(offset_of!(Bank, pin_output_clear), 0x24);
        assert_eq!(offset_of!(Bank, rising_edge_detect_enable), 0x30);
        assert_eq!(offset_of!(Bank, falling_edge_detect_enable), 0x3c);
        assert_eq!(offset_of!(Bank, edge_detect_status), 0x48);
        assert_eq!(offset_of!(Bank, direction_set), 0x54);
        assert_eq!(offset_of!(Bank, direction_clear), 0x60);
        assert_eq!(offset_of!(Bank, rising_edge_detect_set), 0x6c);
        assert_eq!(offset_of!(Bank, rising_edge_detect_clear), 0x78);
        assert_eq!(offset_of!(Bank, falling_edge_detect_set), 0x84);
        assert_eq!(offset_of!(Bank, falling_edge_detect_clear), 0x90);
        assert_eq!(offset_of!(Bank, ap_interrupt_mask), 0x9c);
        assert_eq!(offset_of!(Bank, cp_interrupt_mask), 0xa8);
        assert_eq!(size_of::<Bank>(), 0xac);
        assert_eq!(align_of::<Bank>(), 4);
    }

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, pin_level), 0x00);
        assert_eq!(offset_of!(RegisterBlock, pin_direction), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, pin_output_set), 0x18);
        assert_eq!(offset_of!(RegisterBlock, pin_output_clear), 0x24);
        assert_eq!(offset_of!(RegisterBlock, rising_edge_detect_enable), 0x30);
        assert_eq!(offset_of!(RegisterBlock, falling_edge_detect_enable), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, edge_detect_status), 0x48);
        assert_eq!(offset_of!(RegisterBlock, direction_set), 0x54);
        assert_eq!(offset_of!(RegisterBlock, direction_clear), 0x60);
        assert_eq!(offset_of!(RegisterBlock, rising_edge_detect_set), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, rising_edge_detect_clear), 0x78);
        assert_eq!(offset_of!(RegisterBlock, falling_edge_detect_set), 0x84);
        assert_eq!(offset_of!(RegisterBlock, falling_edge_detect_clear), 0x90);
        assert_eq!(offset_of!(RegisterBlock, ap_interrupt_mask), 0x9c);
        assert_eq!(offset_of!(RegisterBlock, cp_interrupt_mask), 0xa8);
        assert_eq!(offset_of!(RegisterBlock, gpio3), 0x100);
        assert_eq!(size_of::<RegisterBlock>(), 0x1ac);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn bank_mapping() {
        let (gpio, _) = fixture();
        let base = &gpio as *const _ as usize;
        for (i, offset) in [0, 4, 8, 0x100].into_iter().enumerate() {
            let bank = gpio.bank(i as u8).unwrap();
            assert_eq!(bank.pin_level() as *const _ as usize - base, offset);
            assert_eq!(
                bank.pin_output_set() as *const _ as usize - base,
                offset + 24
            );
            assert_eq!(
                bank.pin_output_clear() as *const _ as usize - base,
                offset + 36
            );
            assert_eq!(
                bank.direction_clear() as *const _ as usize - base,
                offset + 96
            );
            assert_eq!(
                bank.direction_set() as *const _ as usize - base,
                offset + 0x54
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
                let pad = FlexPad::__new_k1(n, &gpio, &mfpr).into_function::<7>();
                assert_eq!(config.read(), 0xffff_ffff);
                assert_eq!(aliases.map(|reg| read(reg)), [0; 4]);
                let pad = FlexPad::from(pad).into_function::<0>();
                assert_eq!(config.read(), 0xffff_fff8);

                let mut output = FlexPad::from(pad).into_output(PinState::High);
                let function = match n {
                    70..=73 | 93..=103 => 1,
                    104..=109 => 4,
                    _ => 0,
                };
                assert_eq!(config.read(), 0xffff_fff8 | function);
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
        let _ = unsafe { FlexPad::__new_k1(128, &gpio, &mfpr) };
    }

    #[test]
    #[should_panic(expected = "GPIO alternate function must be in 0..8")]
    fn invalid_function() {
        let (gpio, mfpr) = fixture();
        // SAFETY: Exclusive ownership of this RAM-backed pad.
        let _ = unsafe { FlexPad::__new_k1(0, &gpio, &mfpr) }.into_function::<8>();
    }
}
