//! K1/M1 CAN-FD controller registers.

use crate::register::{RC, RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/net/can/flexcan/flexcan-core.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/arch/riscv/boot/dts/spacemit/k1.dtsi
// Only the register subset exercised by the SpacemiT FlexCAN driver is exposed.
// K1 uses the driver's two 512-byte message banks and 64 acceptance masks.

/// K1/M1 CAN-FD controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Module configuration and soft reset.
    pub module_control: RWNoModify<u32>,
    /// Nominal bit timing and protocol control.
    pub control: RW<u32>,
    /// Timestamp counter; reading unlocks the receive mailbox.
    pub timer: RC<u32>,
    _padding_0x00c: [u32; 1],
    /// Global receive mask.
    pub receive_global_mask: RW<u32>,
    /// Mailbox 14 receive mask.
    pub receive14_mask: RW<u32>,
    /// Mailbox 15 receive mask.
    pub receive15_mask: RW<u32>,
    /// Read-only view of protocol error counters.
    pub error_counter: RO<u32>,
    /// Protocol status, read-clear errors and W1C interrupts.
    pub error_status: RWNoModify<u32>,
    /// Mailbox 32..63 interrupt enables.
    pub interrupt_mask2: RW<u32>,
    /// Mailbox 0..31 interrupt enables.
    pub interrupt_mask1: RW<u32>,
    /// Mailbox 32..63 interrupt flags.
    pub interrupt_flags2: RW1C<u32>,
    /// Mailbox 0..31 interrupt flags.
    pub interrupt_flags1: RW1C<u32>,
    /// Mailbox, FIFO and memory-error control.
    pub control2: RW<u32>,
    _padding_0x038: [u32; 4],
    /// Receive FIFO global mask.
    pub receive_fifo_mask: RW<u32>,
    _padding_0x04c: [u32; 1],
    /// Extended nominal bit timing.
    pub bit_timing: RW<u32>,
    _padding_0x054: [u32; 11],
    /// Driver-supported message banks; access follows the active mailbox layout.
    pub message_ram: [[RWNoModify<u32>; 128]; 2],
    _padding_0x480: [u32; 256],
    /// Driver-supported individual receive masks.
    pub receive_individual_mask: [RW<u32>; 64],
    _padding_0x980: [u32; 88],
    /// Memory-error control and write lock.
    pub memory_error_control: RW<u32>,
    _padding_0xae4: [u32; 71],
    /// CAN-FD format, delay compensation and status.
    pub fd_control: RWNoModify<u32>,
    /// CAN-FD data-phase bit timing.
    pub fd_bit_timing: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, module_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, control), 0x4);
        assert_eq!(offset_of!(RegisterBlock, timer), 0x8);
        assert_eq!(offset_of!(RegisterBlock, receive_global_mask), 0x10);
        assert_eq!(offset_of!(RegisterBlock, receive14_mask), 0x14);
        assert_eq!(offset_of!(RegisterBlock, receive15_mask), 0x18);
        assert_eq!(offset_of!(RegisterBlock, error_counter), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, error_status), 0x20);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask2), 0x24);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask1), 0x28);
        assert_eq!(offset_of!(RegisterBlock, interrupt_flags2), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_flags1), 0x30);
        assert_eq!(offset_of!(RegisterBlock, control2), 0x34);
        assert_eq!(offset_of!(RegisterBlock, receive_fifo_mask), 0x48);
        assert_eq!(offset_of!(RegisterBlock, bit_timing), 0x50);
        assert_eq!(offset_of!(RegisterBlock, message_ram), 0x80);
        assert_eq!(offset_of!(RegisterBlock, receive_individual_mask), 0x880);
        assert_eq!(offset_of!(RegisterBlock, memory_error_control), 0xae0);
        assert_eq!(offset_of!(RegisterBlock, fd_control), 0xc00);
        assert_eq!(offset_of!(RegisterBlock, fd_bit_timing), 0xc04);
        assert_eq!(size_of::<RegisterBlock>(), 0xc08);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
