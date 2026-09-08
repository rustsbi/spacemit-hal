//! K1/M1 One-Wire bus master registers.

use crate::register::{RC, RWNoModify};
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/16.Low-Speed_Interface_System.md#1653-register-description

/// K1/M1 One-Wire bus master registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Bus commands and pin state.
    pub command: RWNoModify<u32>,
    /// Transmit and receive buffer.
    pub data: RWNoModify<u32>,
    /// Reading acknowledges presence detection and the interrupt output.
    pub interrupt_status: RC<u32>,
    /// Interrupt and direct-drive enables.
    pub interrupt_enable: RW<u32>,
    /// Prescaler and clock divider.
    pub clock_divider: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, command), 0x000);
        assert_eq!(offset_of!(RegisterBlock, data), 0x004);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x008);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, clock_divider), 0x010);
        assert_eq!(size_of::<RegisterBlock>(), 0x014);
    }
}
