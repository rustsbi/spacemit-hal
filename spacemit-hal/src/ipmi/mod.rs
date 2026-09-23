//! IPMI keyboard-controller-style (KCS) interface on the V100 platform.

use crate::register::RWNoModify;

/// Two byte-wide KCS registers (`IPI0001`, register spacing one).
///
/// Reads of the data register consume response data. Writes of either register
/// advance the KCS protocol; neither register supports read-modify-write.
#[repr(C)]
pub struct RegisterBlock {
    /// BMC response data (read) or host request data (write).
    pub data: RWNoModify<u8>,
    /// Interface status (read) or KCS command (write).
    pub status_command: RWNoModify<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, data), 0);
        assert_eq!(offset_of!(RegisterBlock, status_command), 1);
        assert_eq!(size_of::<RegisterBlock>(), 2);
        assert_eq!(align_of::<RegisterBlock>(), 1);
    }
}
