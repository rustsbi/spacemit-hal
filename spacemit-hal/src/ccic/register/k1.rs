//! K1 camera capture and CSI PHY registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/media/platform/spacemit/camera/cam_ccic/ccic_hwreg.h
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/media/platform/spacemit/camera/cam_ccic/ccic_hwreg.c
// The CSI PHY and CCIC device-tree nodes alias this single register block.

/// K1 camera capture and CSI PHY registers.
#[repr(C)]
pub struct RegisterBlock {
    /// REG_Y0BAR.
    pub y0bar: RW<u32>,
    _padding_0x004: [u32; 2],
    /// REG_U0BAR.
    pub u0bar: RW<u32>,
    _padding_0x010: [u32; 2],
    /// REG_V0BAR.
    pub v0bar: RW<u32>,
    _padding_0x01c: [u32; 2],
    /// REG_IMGPITCH.
    pub imgpitch: RW<u32>,
    /// REG_IRQSTATRAW.
    pub irqstatraw: RO<u32>,
    /// REG_IRQMASK.
    pub irqmask: RW<u32>,
    /// REG_IRQSTAT.
    pub irqstat: RW1C<u32>,
    /// REG_IMGSIZE.
    pub imgsize: RW<u32>,
    /// REG_IMGOFFSET.
    pub imgoffset: RW<u32>,
    /// REG_CTRL0.
    pub ctrl0: RWNoModify<u32>,
    /// REG_CTRL1.
    pub ctrl1: RWNoModify<u32>,
    /// REG_CTRL2.
    pub ctrl2: RWNoModify<u32>,
    /// REG_CTRL3.
    pub ctrl3: RWNoModify<u32>,
    _padding_0x04c: [u32; 5],
    /// REG_LNNUM.
    pub lnnum: RO<u32>,
    _padding_0x064: [u32; 39],
    /// REG_CSI2_CTRL0.
    pub csi2_ctrl0: RWNoModify<u32>,
    _padding_0x104: [u32; 4],
    /// REG_CSI2_VCCTRL.
    pub csi2_vcctrl: RW<u32>,
    _padding_0x118: [u32; 1],
    /// REG_CSI2_DT_FLT.
    pub csi2_dt_flt: RW<u32>,
    _padding_0x120: [u32; 1],
    /// REG_CSI2_DPHY1.
    pub csi2_dphy1: RW<u32>,
    /// REG_CSI2_DPHY2.
    pub csi2_dphy2: RW<u32>,
    /// REG_CSI2_DPHY3.
    pub csi2_dphy3: RW<u32>,
    /// REG_CSI2_DPHY4.
    pub csi2_dphy4: RWNoModify<u32>,
    /// REG_CSI2_DPHY5.
    pub csi2_dphy5: RW<u32>,
    /// REG_CSI2_DPHY6.
    pub csi2_dphy6: RW<u32>,
    _padding_0x13c: [u32; 1],
    /// REG_CSI2_CTRL2.
    pub csi2_ctrl2: RWNoModify<u32>,
    /// REG_CSI2_CTRL3.
    pub csi2_ctrl3: RWNoModify<u32>,
    _padding_0x148: [u32; 61],
    /// REG_FRAME_CNT.
    pub frame_cnt: RO<u32>,
    _padding_0x240: [u32; 52],
    /// REG_IDI_CTRL.
    pub idi_ctrl: RWNoModify<u32>,
    _padding_0x314: [u32; 7],
    /// REG_IDI_TRIG_LINE_NUM.
    pub idi_trig_line_num: RW<u32>,
    /// REG_CSI_LANE_STATE_DBG.
    pub csi_lane_state_dbg: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, y0bar), 0x000);
        assert_eq!(offset_of!(RegisterBlock, u0bar), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, v0bar), 0x018);
        assert_eq!(offset_of!(RegisterBlock, imgpitch), 0x024);
        assert_eq!(offset_of!(RegisterBlock, irqstatraw), 0x028);
        assert_eq!(offset_of!(RegisterBlock, irqmask), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, irqstat), 0x030);
        assert_eq!(offset_of!(RegisterBlock, imgsize), 0x034);
        assert_eq!(offset_of!(RegisterBlock, imgoffset), 0x038);
        assert_eq!(offset_of!(RegisterBlock, ctrl0), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, ctrl1), 0x040);
        assert_eq!(offset_of!(RegisterBlock, ctrl2), 0x044);
        assert_eq!(offset_of!(RegisterBlock, ctrl3), 0x048);
        assert_eq!(offset_of!(RegisterBlock, lnnum), 0x060);
        assert_eq!(offset_of!(RegisterBlock, csi2_ctrl0), 0x100);
        assert_eq!(offset_of!(RegisterBlock, csi2_vcctrl), 0x114);
        assert_eq!(offset_of!(RegisterBlock, csi2_dt_flt), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy1), 0x124);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy2), 0x128);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy3), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy4), 0x130);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy5), 0x134);
        assert_eq!(offset_of!(RegisterBlock, csi2_dphy6), 0x138);
        assert_eq!(offset_of!(RegisterBlock, csi2_ctrl2), 0x140);
        assert_eq!(offset_of!(RegisterBlock, csi2_ctrl3), 0x144);
        assert_eq!(offset_of!(RegisterBlock, frame_cnt), 0x23c);
        assert_eq!(offset_of!(RegisterBlock, idi_ctrl), 0x310);
        assert_eq!(offset_of!(RegisterBlock, idi_trig_line_num), 0x330);
        assert_eq!(offset_of!(RegisterBlock, csi_lane_state_dbg), 0x334);
        assert_eq!(size_of::<RegisterBlock>(), 0x338);
    }
}
