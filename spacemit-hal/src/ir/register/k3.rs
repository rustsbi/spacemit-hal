//! K3 infrared receiver registers.

use crate::register::{RC, RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/ir_rx.md

/// K3 infrared receiver registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Receiver enable (IRC_EN).
    pub enable: RW<u32>,
    /// Working-clock divider (CLKDIV).
    pub clock_divider: RW<u32>,
    /// Noise rejection threshold (NOISETHR).
    pub noise_threshold: RW<u32>,
    /// Hardware idle state; writing one sets idle (IDLE_STATE).
    pub idle_state: RWNoModify<u32>,
    /// Reading pops receive FIFO data (FIFO_OUT).
    pub fifo_data: RC<u32>,
    /// Receive FIFO flags and count (FIFO_STS).
    pub fifo_status: RO<u32>,
    /// Receive FIFO interrupt threshold (FIFO_CMP).
    pub fifo_compare: RW<u32>,
    /// Interrupt enables (INT_EN).
    pub interrupt_enable: RW<u32>,
    /// Write-one-to-clear interrupt flags (INT_FLAG).
    pub interrupt_flags: RW1C<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, enable), 0x0);
        assert_eq!(offset_of!(RegisterBlock, clock_divider), 0x4);
        assert_eq!(offset_of!(RegisterBlock, noise_threshold), 0x8);
        assert_eq!(offset_of!(RegisterBlock, idle_state), 0xc);
        assert_eq!(offset_of!(RegisterBlock, fifo_data), 0x10);
        assert_eq!(offset_of!(RegisterBlock, fifo_status), 0x14);
        assert_eq!(offset_of!(RegisterBlock, fifo_compare), 0x18);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_flags), 0x20);
        assert_eq!(size_of::<RegisterBlock>(), 0x24);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
