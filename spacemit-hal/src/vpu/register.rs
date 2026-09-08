//! K1/M1 and K3 VPU host registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// Both vendor mvx_lsid.h headers set MVX_LSID_MAX to four.

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/media/platform/spacemit/vpu_k1x/dev/mvx_hwreg.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/media/platform/spacemit/vpu_k3/dev/mvx_hwreg.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/media/platform/spacemit/vpu_k3/dev/mvx_lsid.c

/// K1/M1 and K3 VPU host registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Hardware identifier.
    pub hardware_id: RO<u32>,
    /// Engine enable.
    pub enable: RW<u32>,
    /// Hardware core count.
    pub core_count: RO<u32>,
    /// Logical-session count.
    pub session_count: RO<u32>,
    /// Active sessions per core.
    pub core_session: RO<u32>,
    /// Hardware job queue.
    pub job_queue: RWNoModify<u32>,
    /// Video-engine interrupt summary.
    pub interrupt_video: RO<u32>,
    _padding_0x01c: [u32; 2],
    /// Clock override.
    pub force_clock: RW<u32>,
    _padding_0x028: [u32; 3],
    /// Fused capabilities.
    pub fuse: RO<u32>,
    /// Hardware configuration.
    pub configuration: RO<u32>,
    _padding_0x03c: [u32; 1],
    /// Protection configuration.
    pub protection: RO<u32>,
    _padding_0x044: [u32; 3],
    /// Engine reset.
    pub reset: RWNoModify<u32>,
    _padding_0x054: [u32; 107],
    /// Logical sessions; limit use to session_count.
    pub session: [Session; 4],
}

/// VPU logical-session registers.
#[repr(C)]
pub struct Session {
    /// Core allocation policy.
    pub control: RW<u32>,
    /// Page table and MMU control.
    pub mmu_control: RW<u32>,
    /// Protection status.
    pub non_protected: RO<u32>,
    /// Session allocation.
    pub allocation: RW<u32>,
    /// MMU flush command.
    pub flush_all: RWNoModify<u32>,
    /// Scheduling control.
    pub schedule: RWNoModify<u32>,
    /// Termination command and status.
    pub terminate: RWNoModify<u32>,
    /// Video-engine interrupt acknowledgement.
    pub interrupt_video: RWNoModify<u32>,
    /// Host-to-firmware interrupt.
    pub interrupt_host: RWNoModify<u32>,
    /// Interrupt signal state.
    pub interrupt_signal: RO<u32>,
    _padding_0x028: [u32; 1],
    /// Stream identifier.
    pub stream_id: RO<u32>,
    /// Bus attributes.
    pub bus_attributes: [RW<u32>; 4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, hardware_id), 0x000);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x004);
        assert_eq!(offset_of!(RegisterBlock, core_count), 0x008);
        assert_eq!(offset_of!(RegisterBlock, session_count), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, core_session), 0x010);
        assert_eq!(offset_of!(RegisterBlock, job_queue), 0x014);
        assert_eq!(offset_of!(RegisterBlock, interrupt_video), 0x018);
        assert_eq!(offset_of!(RegisterBlock, force_clock), 0x024);
        assert_eq!(offset_of!(RegisterBlock, fuse), 0x034);
        assert_eq!(offset_of!(RegisterBlock, configuration), 0x038);
        assert_eq!(offset_of!(RegisterBlock, protection), 0x040);
        assert_eq!(offset_of!(RegisterBlock, reset), 0x050);
        assert_eq!(offset_of!(RegisterBlock, session), 0x200);
        assert_eq!(size_of::<RegisterBlock>(), 0x300);
        assert_eq!(size_of::<Session>(), 0x40);
        assert_eq!(offset_of!(Session, interrupt_video), 0x1c);
        assert_eq!(offset_of!(Session, bus_attributes), 0x30);
    }
}
