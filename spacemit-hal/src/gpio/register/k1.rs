//! GPIO register layout used by SpacemiT K1 and M1 SoCs.
//!
//! GPIO0 through GPIO2 are interleaved in the main register area. GPIO3 uses
//! the same per-bank register offsets in a separate area at offset `0x100`.

use super::ReadWriteOneToClear;
use volatile_register::{RO, RW, WO};

/// Register group for K1/M1 GPIO3.
#[repr(C)]
pub struct Bank {
    /// Pin level register (`GPIO_PLR`).
    pub pin_level: RO<u32>,
    _reserved0: [u32; 2],
    /// Pin direction register (`GPIO_PDR`).
    pub pin_direction: RW<u32>,
    _reserved1: [u32; 2],
    /// Atomic pin output-set register (`GPIO_PSR`).
    pub pin_output_set: WO<u32>,
    _reserved2: [u32; 2],
    /// Atomic pin output-clear register (`GPIO_PCR`).
    pub pin_output_clear: WO<u32>,
    _reserved3: [u32; 2],
    /// Rising-edge detect enable register (`GPIO_RER`).
    pub rising_edge_detect_enable: RW<u32>,
    _reserved4: [u32; 2],
    /// Falling-edge detect enable register (`GPIO_FER`).
    pub falling_edge_detect_enable: RW<u32>,
    _reserved5: [u32; 2],
    /// Edge-detect status register (`GPIO_EDR`, read/write-one-to-clear).
    pub edge_detect_status: ReadWriteOneToClear,
    _reserved6: [u32; 2],
    /// Atomic direction-set register (`GPIO_SDR`).
    pub direction_set: WO<u32>,
    _reserved7: [u32; 2],
    /// Atomic direction-clear register (`GPIO_CDR`).
    pub direction_clear: WO<u32>,
    _reserved8: [u32; 2],
    /// Atomic rising-edge-enable-set register (`GPIO_SRER`).
    pub rising_edge_detect_set: WO<u32>,
    _reserved9: [u32; 2],
    /// Atomic rising-edge-enable-clear register (`GPIO_CRER`).
    pub rising_edge_detect_clear: WO<u32>,
    _reserved10: [u32; 2],
    /// Atomic falling-edge-enable-set register (`GPIO_SFER`).
    pub falling_edge_detect_set: WO<u32>,
    _reserved11: [u32; 2],
    /// Atomic falling-edge-enable-clear register (`GPIO_CFER`).
    pub falling_edge_detect_clear: WO<u32>,
    _reserved12: [u32; 2],
    /// Application-processor interrupt mask register (`GPIO_APMASK`).
    pub ap_interrupt_mask: RW<u32>,
    _reserved13: [u32; 2],
    /// Companion-processor interrupt mask register (`GPIO_CPMASK`).
    pub cp_interrupt_mask: RW<u32>,
}

/// K1/M1 GPIO register block.
///
/// The array-valued fields contain GPIO0, GPIO1, and GPIO2 in that order.
/// GPIO3 is represented by [`gpio3`](Self::gpio3).
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
    pub edge_detect_status: [ReadWriteOneToClear; 3],
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
    _reserved0: [u32; 19],
    /// GPIO3 registers.
    pub gpio3: Bank,
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
}
