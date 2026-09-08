//! K1/M1 interprocessor mailbox registers.

use crate::register::RWNoModify;
use volatile_register::{RO, WO};

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/9.Top_System.md#984-register-description
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/mailbox/spacemit/k1x_mailbox.h
// IIR is at 0x10 in the vendor structure, not the manual's repeated 0x0c.

/// K1/M1 interprocessor mailbox registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Remote interrupt-set value.
    pub remote_interrupt: RO<u32>,
    /// Outgoing message word.
    pub write_data: WO<u32>,
    /// Remote interrupt requests.
    pub interrupt_set: WO<u32>,
    /// Incoming interrupt acknowledgement.
    pub interrupt_clear: WO<u32>,
    /// Write zero to latch incoming interrupts before reading.
    pub interrupt_identification: RWNoModify<u32>,
    /// Latched incoming message word.
    pub read_data: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, remote_interrupt), 0x000);
        assert_eq!(offset_of!(RegisterBlock, write_data), 0x004);
        assert_eq!(offset_of!(RegisterBlock, interrupt_set), 0x008);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_identification), 0x010);
        assert_eq!(offset_of!(RegisterBlock, read_data), 0x014);
        assert_eq!(size_of::<RegisterBlock>(), 0x018);
    }
}
