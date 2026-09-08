//! K3 high-dimensional dma registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/16_peripherals/hdma.md
// This is the documented HDMA interface, not the separate HSDMA/DMA350 interfaces.

/// K3 high-dimensional dma registers.
#[repr(C)]
pub struct RegisterBlock {
    /// VERSION.
    pub version: RO<u32>,
    /// PERIPHERAL ID.
    pub peripheral_id: RO<u32>,
    /// SCRATCH.
    pub scratch: RW<u32>,
    /// IDENTIFICATION.
    pub identification: RO<u32>,
    /// INTERFACE DESCRIPTION 1.
    pub interface_description_1: RO<u32>,
    /// INTERFACE DESCRIPTION 2.
    pub interface_description_2: RO<u32>,
    _padding_0x018: [u32; 26],
    /// IRQ MASK.
    pub irq_mask: RW<u32>,
    /// IRQ PENDING.
    pub irq_pending: RW1C<u32>,
    /// IRQ SOURC.
    pub irq_source: RO<u32>,
    _padding_0x08c: [u32; 221],
    /// CONTROL.
    pub control: RW<u32>,
    /// TRANSFER ID.
    pub transfer_id: RO<u32>,
    /// Write one to queue a transfer (TRANSFER_SUBMIT).
    pub transfer_submit: RWNoModify<u32>,
    /// FLAGS.
    pub flags: RW<u32>,
    /// DEST ADDRESS.
    pub dest_address: RW<u32>,
    /// SRC ADDRESS.
    pub src_address: RW<u32>,
    /// X LENGTH.
    pub x_length: RW<u32>,
    /// Y LENGTH.
    pub y_length: RW<u32>,
    /// DEST STRIDE.
    pub dest_stride: RW<u32>,
    /// SRC STRIDE.
    pub src_stride: RW<u32>,
    /// TRANSFER DONE.
    pub transfer_done: RO<u32>,
    /// ACTIVE TRANSFER ID.
    pub active_transfer_id: RO<u32>,
    _padding_0x430: [u32; 1],
    /// CURRENT DEST ADDRESS.
    pub current_dest_address: RO<u32>,
    /// CURRENT SRC ADDRESS.
    pub current_src_address: RO<u32>,
    _padding_0x43c: [u32; 3],
    /// TRANSFER PROGRESS.
    pub transfer_progress: RO<u32>,
    /// PARTIAL TRANSFER LENGTH. Unsupported in the documented K3 configuration.
    pub partial_transfer_length: RO<u32>,
    /// PARTIAL TRANSFER ID. Unsupported in the documented K3 configuration.
    pub partial_transfer_id: RO<u32>,
    /// DESCRIPTOR ID.
    pub descriptor_id: RO<u32>,
    /// FRAMELOCK CONFIG. Unsupported in the documented K3 configuration.
    pub framelock_config: RW<u32>,
    /// FRAMELOCK STRIDE. Unsupported in the documented K3 configuration.
    pub framelock_stride: RW<u32>,
    _padding_0x460: [u32; 7],
    /// SG ADDRESS.
    pub sg_address: RW<u32>,
    _padding_0x480: [u32; 4],
    /// DEST ADDRESS HIGH.
    pub dest_address_high: RW<u32>,
    /// SRC ADDRESS HIGH.
    pub src_address_high: RW<u32>,
    /// CURRENT DEST ADDRESS HIGH.
    pub current_dest_address_high: RO<u32>,
    /// CURRENT SRC ADDRESS HIGH.
    pub current_src_address_high: RO<u32>,
    _padding_0x4a0: [u32; 7],
    /// SG ADDRESS HIGH.
    pub sg_address_high: RW<u32>,
    _padding_0x4c0: [u32; 80],
    /// SGDG CFG.
    pub sgdg_cfg: RW<u32>,
    /// PAD VALUE.
    pub pad_value: RW<u32>,
    /// SGDG M SIZE.
    pub sgdg_m_size: RW<u32>,
    /// SGDG K SIZE.
    pub sgdg_k_size: RW<u32>,
    /// SGDG MR SIZE.
    pub sgdg_mr_size: RW<u32>,
    /// SGDG KR SIZE.
    pub sgdg_kr_size: RW<u32>,
    /// SGDG MP SIZE.
    pub sgdg_mp_size: RW<u32>,
    /// SGDG KP SIZE.
    pub sgdg_kp_size: RW<u32>,
    _padding_0x620: [u32; 1],
    /// SGDG MB SIZE.
    pub sgdg_mb_size: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, version), 0x0);
        assert_eq!(offset_of!(RegisterBlock, peripheral_id), 0x4);
        assert_eq!(offset_of!(RegisterBlock, scratch), 0x8);
        assert_eq!(offset_of!(RegisterBlock, identification), 0xc);
        assert_eq!(offset_of!(RegisterBlock, interface_description_1), 0x10);
        assert_eq!(offset_of!(RegisterBlock, interface_description_2), 0x14);
        assert_eq!(offset_of!(RegisterBlock, irq_mask), 0x80);
        assert_eq!(offset_of!(RegisterBlock, irq_pending), 0x84);
        assert_eq!(offset_of!(RegisterBlock, irq_source), 0x88);
        assert_eq!(offset_of!(RegisterBlock, control), 0x400);
        assert_eq!(offset_of!(RegisterBlock, transfer_id), 0x404);
        assert_eq!(offset_of!(RegisterBlock, transfer_submit), 0x408);
        assert_eq!(offset_of!(RegisterBlock, flags), 0x40c);
        assert_eq!(offset_of!(RegisterBlock, dest_address), 0x410);
        assert_eq!(offset_of!(RegisterBlock, src_address), 0x414);
        assert_eq!(offset_of!(RegisterBlock, x_length), 0x418);
        assert_eq!(offset_of!(RegisterBlock, y_length), 0x41c);
        assert_eq!(offset_of!(RegisterBlock, dest_stride), 0x420);
        assert_eq!(offset_of!(RegisterBlock, src_stride), 0x424);
        assert_eq!(offset_of!(RegisterBlock, transfer_done), 0x428);
        assert_eq!(offset_of!(RegisterBlock, active_transfer_id), 0x42c);
        assert_eq!(offset_of!(RegisterBlock, current_dest_address), 0x434);
        assert_eq!(offset_of!(RegisterBlock, current_src_address), 0x438);
        assert_eq!(offset_of!(RegisterBlock, transfer_progress), 0x448);
        assert_eq!(offset_of!(RegisterBlock, partial_transfer_length), 0x44c);
        assert_eq!(offset_of!(RegisterBlock, partial_transfer_id), 0x450);
        assert_eq!(offset_of!(RegisterBlock, descriptor_id), 0x454);
        assert_eq!(offset_of!(RegisterBlock, framelock_config), 0x458);
        assert_eq!(offset_of!(RegisterBlock, framelock_stride), 0x45c);
        assert_eq!(offset_of!(RegisterBlock, sg_address), 0x47c);
        assert_eq!(offset_of!(RegisterBlock, dest_address_high), 0x490);
        assert_eq!(offset_of!(RegisterBlock, src_address_high), 0x494);
        assert_eq!(offset_of!(RegisterBlock, current_dest_address_high), 0x498);
        assert_eq!(offset_of!(RegisterBlock, current_src_address_high), 0x49c);
        assert_eq!(offset_of!(RegisterBlock, sg_address_high), 0x4bc);
        assert_eq!(offset_of!(RegisterBlock, sgdg_cfg), 0x600);
        assert_eq!(offset_of!(RegisterBlock, pad_value), 0x604);
        assert_eq!(offset_of!(RegisterBlock, sgdg_m_size), 0x608);
        assert_eq!(offset_of!(RegisterBlock, sgdg_k_size), 0x60c);
        assert_eq!(offset_of!(RegisterBlock, sgdg_mr_size), 0x610);
        assert_eq!(offset_of!(RegisterBlock, sgdg_kr_size), 0x614);
        assert_eq!(offset_of!(RegisterBlock, sgdg_mp_size), 0x618);
        assert_eq!(offset_of!(RegisterBlock, sgdg_kp_size), 0x61c);
        assert_eq!(offset_of!(RegisterBlock, sgdg_mb_size), 0x624);
        assert_eq!(size_of::<RegisterBlock>(), 0x628);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
