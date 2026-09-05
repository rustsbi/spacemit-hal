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
    /// Returns a normalized view of one K3 GPIO bank.
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
    fn normalized_banks_select_the_expected_registers() {
        // SAFETY: Every field in the register block is a transparent wrapper
        // around a 32-bit integer, for which the all-zero bit pattern is valid.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };

        let gpio0 = registers.bank(0).unwrap();
        assert!(core::ptr::eq(gpio0.pin_level, &registers.gpio0.pin_level));
        assert!(core::ptr::eq(
            gpio0.direction_set,
            &registers.gpio0.direction_set
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
