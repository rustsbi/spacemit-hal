//! K1/M1 DPU top, control, and interrupt registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_top.h
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_ctl.h
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_intp.h
// Pipeline, compositor, MMU, and writeback blocks are not represented here.
// Clock gates: dpu_crg.h; interrupt acknowledgements: ../dpu_saturn.c in the same tree.

/// K1/M1 DPU top, control, and interrupt registers.
#[repr(C)]
pub struct RegisterBlock {
    /// dpu_top_reg_0.
    pub hardware_id: RO<u32>,
    _padding_0x004: [u32; 213],
    /// dpu_top_reg_214.
    pub pipeline_status0: RO<u32>,
    /// dpu_top_reg_215.
    pub pipeline_status1: RO<u32>,
    /// dpu_top_reg_216.
    pub pipeline_status2: RO<u32>,
    /// dpu_top_reg_217.
    pub pipeline_status3: RO<u32>,
    _padding_0x368: [u32; 102],
    /// Scene and command-list control.
    pub control: Control,
    _padding_0x678: [u32; 34],
    /// Internal automatic clock gates.
    pub clock_gates: [RW<u32>; 5],
    _padding_0x714: [u32; 123],
    /// Display interrupt registers.
    pub interrupt: Interrupt,
}

/// DPU control registers.
#[repr(C)]
pub struct Control {
    /// ctl0_nml_rch_en, ctl0_nml_scl_en, ctl0_nml_wb_en, ctl0_nml_outctl_en.
    pub dpu_ctl_reg_0: RW<u32>,
    _padding_0x004: [u32; 1],
    /// ctl0_nml_cmd_updt_en.
    pub dpu_ctl_reg_2: RW<u32>,
    /// ctl0_nml_cfg_rdy, ctl0_sw_clr.
    pub dpu_ctl_reg_3: RWNoModify<u32>,
    _padding_0x010: [u32; 1],
    /// ctl0_secu_rch_en, ctl0_secu_scl_en, ctl0_secu_wb_en.
    pub dpu_ctl_reg_5: RW<u32>,
    _padding_0x018: [u32; 1],
    /// ctl0_secu_cmd_updt_en.
    pub dpu_ctl_reg_7: RW<u32>,
    /// ctl0_secu_cfg_rdy.
    pub dpu_ctl_reg_8: RWNoModify<u32>,
    _padding_0x024: [u32; 1],
    /// ctl0_video_mod, ctl0_dbg_mod, ctl0_timing_inter0, ctl0_timing_inter1.
    pub dpu_ctl_reg_10: RW<u32>,
    /// ctl0_sw_start, ctl0_dbg_unflow_clr.
    pub dpu_ctl_reg_11: RWNoModify<u32>,
    /// ctl1_nml_rch_en, ctl1_nml_scl_en, ctl1_nml_wb_en, ctl1_nml_outctl_en.
    pub dpu_ctl_reg_12: RW<u32>,
    _padding_0x034: [u32; 1],
    /// ctl1_nml_cmd_updt_en.
    pub dpu_ctl_reg_14: RW<u32>,
    /// ctl1_nml_cfg_rdy, ctl1_sw_clr.
    pub dpu_ctl_reg_15: RWNoModify<u32>,
    _padding_0x040: [u32; 1],
    /// ctl1_secu_rch_en, ctl1_secu_scl_en, ctl1_secu_wb_en.
    pub dpu_ctl_reg_17: RW<u32>,
    _padding_0x048: [u32; 1],
    /// ctl1_secu_cmd_updt_en.
    pub dpu_ctl_reg_19: RW<u32>,
    /// ctl1_secu_cfg_rdy.
    pub dpu_ctl_reg_20: RWNoModify<u32>,
    _padding_0x054: [u32; 1],
    /// ctl1_video_mod, ctl1_dbg_mod, ctl1_timing_inter0, ctl1_timing_inter1.
    pub dpu_ctl_reg_22: RW<u32>,
    /// ctl1_sw_start, ctl1_dbg_unflow_clr.
    pub dpu_ctl_reg_23: RWNoModify<u32>,
    /// ctl2_nml_rch_en, ctl2_nml_scl_en, ctl2_nml_wb_en, ctl2_nml_outctl_en.
    pub dpu_ctl_reg_24: RW<u32>,
    _padding_0x064: [u32; 1],
    /// ctl2_nml_cmd_updt_en.
    pub dpu_ctl_reg_26: RW<u32>,
    /// ctl2_nml_cfg_rdy, ctl2_sw_clr.
    pub dpu_ctl_reg_27: RWNoModify<u32>,
    _padding_0x070: [u32; 1],
    /// ctl2_secu_rch_en, ctl2_secu_scl_en, ctl2_secu_wb_en.
    pub dpu_ctl_reg_29: RW<u32>,
    _padding_0x078: [u32; 1],
    /// ctl2_secu_cmd_updt_en.
    pub dpu_ctl_reg_31: RW<u32>,
    /// ctl2_secu_cfg_rdy.
    pub dpu_ctl_reg_32: RWNoModify<u32>,
    _padding_0x084: [u32; 1],
    /// ctl2_video_mod, ctl2_dbg_mod, ctl2_timing_inter0, ctl2_timing_inter1.
    pub dpu_ctl_reg_34: RW<u32>,
    /// ctl2_sw_start, ctl2_dbg_unflow_clr.
    pub dpu_ctl_reg_35: RWNoModify<u32>,
    /// ctl3_nml_rch_en, ctl3_nml_scl_en, ctl3_nml_wb_en, ctl3_nml_outctl_en.
    pub dpu_ctl_reg_36: RW<u32>,
    _padding_0x094: [u32; 1],
    /// ctl3_nml_cmd_updt_en.
    pub dpu_ctl_reg_38: RW<u32>,
    /// ctl3_nml_cfg_rdy, ctl3_sw_clr.
    pub dpu_ctl_reg_39: RWNoModify<u32>,
    /// ctl3_video_mod, ctl3_dbg_mod, ctl3_timing_inter0, ctl3_timing_inter1.
    pub dpu_ctl_reg_40: RW<u32>,
    /// ctl3_sw_start, ctl3_dbg_unflow_clr.
    pub dpu_ctl_reg_41: RWNoModify<u32>,
    /// ctl4_nml_rch_en, ctl4_nml_scl_en, ctl4_nml_wb_en.
    pub dpu_ctl_reg_42: RW<u32>,
    _padding_0x0ac: [u32; 1],
    /// ctl4_nml_cmd_updt_en.
    pub dpu_ctl_reg_44: RW<u32>,
    /// ctl4_nml_cfg_rdy, ctl4_sw_clr.
    pub dpu_ctl_reg_45: RWNoModify<u32>,
    _padding_0x0b8: [u32; 1],
    /// ctl4_timing_inter0, ctl4_timing_inter1.
    pub dpu_ctl_reg_47: RW<u32>,
    /// ctl_nml_scl0_layer_id, ctl_nml_scl0_layer_right.
    pub dpu_ctl_reg_48: RW<u32>,
    /// ctl_nml_scl1_layer_id, ctl_nml_scl1_layer_right.
    pub dpu_ctl_reg_49: RW<u32>,
    /// ctl_nml_scl2_layer_id, ctl_nml_scl2_layer_right.
    pub dpu_ctl_reg_50: RW<u32>,
    /// ctl_nml_scl3_layer_id, ctl_nml_scl3_layer_right.
    pub dpu_ctl_reg_51: RW<u32>,
    /// ctl_secu_scl0_layer_id, ctl_secu_scl0_layer_right.
    pub dpu_ctl_reg_52: RW<u32>,
    /// ctl_secu_scl1_layer_id, ctl_secu_scl1_layer_right.
    pub dpu_ctl_reg_53: RW<u32>,
    /// ctl_secu_scl2_layer_id, ctl_secu_scl2_layer_right.
    pub dpu_ctl_reg_54: RW<u32>,
    /// ctl_secu_scl3_layer_id, ctl_secu_scl3_layer_right.
    pub dpu_ctl_reg_55: RW<u32>,
    /// outctl_secu, cmps_secu, prc_curve_secu.
    pub dpu_ctl_reg_56: RW<u32>,
    /// ctl_rd_shadow.
    pub dpu_ctl_reg_57: RW<u32>,
    /// rch_conflict_ints, scl_conflict_ints, wb_timeout_ints.
    pub dpu_ctl_reg_58: RWNoModify<u32>,
    /// rch_conflict_ints_msk, scl_conflict_int_msk, wb_timeout_int_msk.
    pub dpu_ctl_reg_59: RW<u32>,
    /// rch_conflict_int_raw, scl_conflict_int_raw, wb_timeout_int_raw.
    pub dpu_ctl_reg_60: RO<u32>,
    /// ctl_nml_reuse_scl0_en, ctl_nml_cmps_scl0_en.
    pub dpu_ctl_reg_61: RW<u32>,
    /// ctl_nml_reuse_scl1_en, ctl_nml_cmps_scl1_en.
    pub dpu_ctl_reg_62: RW<u32>,
    /// ctl_nml_reuse_scl2_en, ctl_nml_cmps_scl2_en.
    pub dpu_ctl_reg_63: RW<u32>,
    /// ctl_nml_reuse_scl3_en, ctl_nml_cmps_scl3_en.
    pub dpu_ctl_reg_64: RW<u32>,
    /// ctl_secu_reuse_scl0_en, ctl_secu_cmps_scl0_en.
    pub dpu_ctl_reg_65: RW<u32>,
    /// ctl_secu_reuse_scl1_en, ctl_secu_cmps_scl1_en.
    pub dpu_ctl_reg_66: RW<u32>,
    /// ctl_secu_reuse_scl2_en, ctl_secu_cmps_scl2_en.
    pub dpu_ctl_reg_67: RW<u32>,
    /// ctl_secu_reuse_scl3_en, ctl_secu_cmps_scl3_en.
    pub dpu_ctl_reg_68: RW<u32>,
    /// ctl_nml_cmdlist_rch_en.
    pub dpu_ctl_reg_69: [RW<u32>; 12],
    /// ctl_nml_cmdlist_wb_en.
    pub dpu_ctl_reg_81: [RW<u32>; 2],
    /// ctl_nml_cmdlist_rch_cfg_rdy, ctl_nml_cmdlist_wb_cfg_rdy.
    pub dpu_ctl_reg_83: RWNoModify<u32>,
    /// ctl_wb0_sel_id, ctl_wb0_sel_right.
    pub dpu_ctl_reg_84: RW<u32>,
    /// ctl_wb1_sel_id, ctl_wb1_sel_right.
    pub dpu_ctl_reg_85: RW<u32>,
    /// ctl_rdma_act, ctl_scl_act, ctl_layer_act.
    pub dpu_ctl_reg_86: RO<u32>,
    /// ctl_cmps_outctl_act, ctl_wb_act, ctl_wb_slice_cnt.
    pub dpu_ctl_reg_87: RO<u32>,
    /// cmdlist_rch_act, cmdlist_wb_act, scene_ctl_dbg0.
    pub dpu_ctl_reg_88: RO<u32>,
    /// scene_ctl_dbg1, scene_ctl_dbg2.
    pub dpu_ctl_reg_89: RO<u32>,
    /// scene_ctl_dbg3, scene_ctl_dbg4.
    pub dpu_ctl_reg_90: RO<u32>,
    /// rdma_clr_req_aclk, wb_clr_req_aclk, rdma_clr_ack_aclk, wb_clr_ack_aclk.
    pub dpu_ctl_reg_91: RO<u32>,
    /// outctl_clr_req_aclk, outctl_clr_ack_aclk, wb_conflict_hld.
    pub dpu_ctl_reg_92: RO<u32>,
    /// cmdlist_clr_req_aclk, cmdlist_clr_ack_aclk.
    pub dpu_ctl_reg_93: RO<u32>,
}

/// DPU interrupt registers.
#[repr(C)]
pub struct Interrupt {
    /// onl0_nml_frm_timing_vsync_int_msk.
    pub dpu_int_reg_0: RW<u32>,
    /// onl0_nml_dma_dbg_int_msk.
    pub dpu_int_reg_1: RW<u32>,
    /// onl1_nml_frm_timing_vsync_int_msk.
    pub dpu_int_reg_2: RW<u32>,
    /// onl1_nml_dma_dbg_int_msk.
    pub dpu_int_reg_3: RW<u32>,
    /// onl2_nml_frm_timing_vsync_int_msk.
    pub dpu_int_reg_4: RW<u32>,
    /// onl2_nml_dma_dbg_int_msk.
    pub dpu_int_reg_5: RW<u32>,
    /// offl0_cfg_rdy_clr_int_msk.
    pub dpu_int_reg_6: RW<u32>,
    /// offl0_nml_dma_dbg_int_msk.
    pub dpu_int_reg_7: RW<u32>,
    /// offl1_cfg_rdy_clr_int_msk.
    pub dpu_int_reg_8: RW<u32>,
    /// offl1_nml_dma_dbg_int_msk.
    pub dpu_int_reg_9: RW<u32>,
    /// onl0_nml_frm_timing_vsync_int_sts.
    pub dpu_int_reg_10: RWNoModify<u32>,
    /// onl0_nml_dma_dbg_int_sts.
    pub dpu_int_reg_11: RWNoModify<u32>,
    /// onl1_nml_frm_timing_vsync_int_sts.
    pub dpu_int_reg_12: RWNoModify<u32>,
    /// onl1_nml_dma_dbg_int_sts.
    pub dpu_int_reg_13: RWNoModify<u32>,
    /// onl2_nml_frm_timing_vsync_int_sts.
    pub dpu_int_reg_14: RW1C<u32>,
    /// onl2_nml_dma_dbg_int_sts.
    pub dpu_int_reg_15: RWNoModify<u32>,
    /// offl0_cfg_rdy_clr_int_sts.
    pub dpu_int_reg_16: RWNoModify<u32>,
    /// offl0_nml_dma_dbg_int_sts.
    pub dpu_int_reg_17: RWNoModify<u32>,
    /// offl1_cfg_rdy_clr_int_sts.
    pub dpu_int_reg_18: RWNoModify<u32>,
    /// offl1_nml_dma_dbg_int_sts.
    pub dpu_int_reg_19: RWNoModify<u32>,
    /// onl0_nml_frm_timing_vsync_int_raw.
    pub dpu_int_reg_20: RO<u32>,
    /// onl0_nml_dma_dbg_int_raw.
    pub dpu_int_reg_21: RO<u32>,
    /// onl1_nml_frm_timing_vsync_int_raw.
    pub dpu_int_reg_22: RO<u32>,
    /// onl1_nml_dma_dbg_int_raw.
    pub dpu_int_reg_23: RO<u32>,
    /// onl2_nml_frm_timing_vsync_int_raw.
    pub dpu_int_reg_24: RO<u32>,
    /// onl2_nml_dma_dbg_int_raw.
    pub dpu_int_reg_25: RO<u32>,
    /// offl0_cfg_rdy_clr_int_raw.
    pub dpu_int_reg_26: RO<u32>,
    /// offl0_nml_dma_dbg_int_raw.
    pub dpu_int_reg_27: RO<u32>,
    /// offl1_cfg_rdy_clr_int_raw.
    pub dpu_int_reg_28: RO<u32>,
    /// offl1_nml_dma_dbg_int_raw.
    pub dpu_int_reg_29: RO<u32>,
    _padding_0x078: [u32; 10],
    /// onl0_secu_frm_timing_vsync_int_msk.
    pub dpu_int_reg_40: RW<u32>,
    /// onl0_secu_dma_dbg_int_msk.
    pub dpu_int_reg_41: RW<u32>,
    /// onl1_secu_frm_timing_vsync_int_msk.
    pub dpu_int_reg_42: RW<u32>,
    /// onl1_secu_dma_dbg_int_msk.
    pub dpu_int_reg_43: RW<u32>,
    /// onl2_secu_frm_timing_vsync_int_msk.
    pub dpu_int_reg_44: RW<u32>,
    /// onl2_secu_dma_dbg_int_msk.
    pub dpu_int_reg_45: RW<u32>,
    /// onl0_secu_frm_timing_vsync_int_sts.
    pub dpu_int_reg_46: RWNoModify<u32>,
    /// onl0_secu_dma_dbg_int_sts.
    pub dpu_int_reg_47: RWNoModify<u32>,
    /// onl1_secu_frm_timing_vsync_int_sts.
    pub dpu_int_reg_48: RWNoModify<u32>,
    /// onl1_secu_dma_dbg_int_sts.
    pub dpu_int_reg_49: RWNoModify<u32>,
    /// onl2_secu_frm_timing_vsync_int_sts.
    pub dpu_int_reg_50: RWNoModify<u32>,
    /// onl2_secu_dma_dbg_int_sts.
    pub dpu_int_reg_51: RWNoModify<u32>,
    /// onl0_secu_frm_timing_vsync_int_raw.
    pub dpu_int_reg_52: RO<u32>,
    /// onl0_secu_dma_dbg_int_raw.
    pub dpu_int_reg_53: RO<u32>,
    /// onl1_secu_frm_timing_vsync_int_raw.
    pub dpu_int_reg_54: RO<u32>,
    /// onl1_secu_dma_dbg_int_raw.
    pub dpu_int_reg_55: RO<u32>,
    /// onl2_secu_frm_timing_vsync_int_raw.
    pub dpu_int_reg_56: RO<u32>,
    /// onl2_secu_dma_dbg_int_raw.
    pub dpu_int_reg_57: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, hardware_id), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pipeline_status0), 0x358);
        assert_eq!(offset_of!(RegisterBlock, pipeline_status1), 0x35c);
        assert_eq!(offset_of!(RegisterBlock, pipeline_status2), 0x360);
        assert_eq!(offset_of!(RegisterBlock, pipeline_status3), 0x364);
        assert_eq!(offset_of!(RegisterBlock, control), 0x500);
        assert_eq!(offset_of!(RegisterBlock, clock_gates), 0x700);
        assert_eq!(offset_of!(RegisterBlock, interrupt), 0x900);
        assert_eq!(size_of::<Control>(), 0x178);
        assert_eq!(size_of::<Interrupt>(), 0xe8);
        assert_eq!(size_of::<RegisterBlock>(), 0x9e8);
    }
}
