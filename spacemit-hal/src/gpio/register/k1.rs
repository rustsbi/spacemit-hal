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
        BankRegisters::new(
            &self.pin_level[index],
            &self.pin_output_set[index],
            &self.pin_output_clear[index],
            &self.direction_set[index],
            &self.direction_clear[index],
        )
    }

    /// Returns a normalized view of one K1/M1 GPIO bank.
    #[inline(always)]
    pub(in crate::gpio) const fn bank(&self, bank: u8) -> Option<BankRegisters<'_>> {
        match bank {
            0 => Some(self.interleaved_bank(0)),
            1 => Some(self.interleaved_bank(1)),
            2 => Some(self.interleaved_bank(2)),
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
        BankRegisters::new(
            &self.pin_level,
            &self.pin_output_set,
            &self.pin_output_clear,
            &self.direction_set,
            &self.direction_clear,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Bank, RegisterBlock};
    use core::mem::{align_of, offset_of, size_of};

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
    fn normalized_banks_select_the_expected_registers() {
        // SAFETY: Every field in the register block is a transparent wrapper
        // around a 32-bit integer, for which the all-zero bit pattern is valid.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };

        let gpio2 = registers.bank(2).unwrap();
        assert!(core::ptr::eq(gpio2.pin_level, &registers.pin_level[2]));
        assert!(core::ptr::eq(
            gpio2.direction_set,
            &registers.direction_set[2]
        ));

        let gpio3 = registers.bank(3).unwrap();
        assert!(core::ptr::eq(gpio3.pin_level, &registers.gpio3.pin_level));
        assert!(core::ptr::eq(
            gpio3.direction_set,
            &registers.gpio3.direction_set
        ));
        assert!(registers.bank(4).is_none());
    }
}
