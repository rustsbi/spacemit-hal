//! K3 inter-processor mailbox registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/16_peripherals/mailbox.md
// Only the 16-channel non-secure interface with two users is represented.

/// K3 inter-processor mailbox registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Mailbox revision.
    pub revision: RO<u32>,
    _padding_0x004: [u32; 3],
    /// FIFO reset requests; completion is self-clearing.
    pub fifo_clear: RWNoModify<u32>,
    _padding_0x014: [u32; 11],
    /// FIFO push/pop ports.
    pub message: [RWNoModify<u32>; 16],
    /// FIFO status and write-one-to-clear access errors.
    pub fifo_status: [RW1C<u32>; 16],
    /// Message counts and write-one-to-clear access errors.
    pub message_status: [RW1C<u32>; 16],
    /// Interrupt registers for the two implemented users.
    pub interrupt: [Interrupt; 2],
    _padding_0x120: [u32; 24],
    /// Four threshold words for each implemented user.
    pub threshold: [[RW<u32>; 4]; 2],
}

/// One mailbox user's interrupt registers.
#[repr(C)]
pub struct Interrupt {
    /// Raw events; writing one sets an event.
    pub raw_status: RWNoModify<u32>,
    /// Masked events; writing one acknowledges an event.
    pub status_clear: RW1C<u32>,
    /// Interrupt enables; writing one enables selected events.
    pub enable_set: RWNoModify<u32>,
    /// Interrupt enables; writing one disables selected events.
    pub enable_clear: RW1C<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, revision), 0x0);
        assert_eq!(offset_of!(RegisterBlock, fifo_clear), 0x10);
        assert_eq!(offset_of!(RegisterBlock, message), 0x40);
        assert_eq!(offset_of!(RegisterBlock, fifo_status), 0x80);
        assert_eq!(offset_of!(RegisterBlock, message_status), 0xc0);
        assert_eq!(offset_of!(RegisterBlock, interrupt), 0x100);
        assert_eq!(offset_of!(RegisterBlock, threshold), 0x180);
        assert_eq!(size_of::<RegisterBlock>(), 0x1a0);
        assert_eq!(align_of::<RegisterBlock>(), 4);
        assert_eq!(offset_of!(Interrupt, raw_status), 0x0);
        assert_eq!(offset_of!(Interrupt, status_clear), 0x4);
        assert_eq!(offset_of!(Interrupt, enable_set), 0x8);
        assert_eq!(offset_of!(Interrupt, enable_clear), 0xc);
        assert_eq!(size_of::<Interrupt>(), 0x10);
        assert_eq!(align_of::<Interrupt>(), 4);
    }
}
