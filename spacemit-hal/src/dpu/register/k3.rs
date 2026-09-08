//! K3 DPU top, control, and interrupt registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_top.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_ctl_top.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/gpu/drm/spacemit/dpu/saturn_regs/dpu_int.h
// Pipeline, compositor, MMU, and writeback blocks are not represented here.
// Interrupt acknowledgements: ../dpu_saturn_hee.c in the same tree.

/// K3 DPU top, control, and interrupt registers.
#[repr(C)]
pub struct RegisterBlock {
    /// saturn_led_top_reg_0.
    pub hardware_id: RO<u32>,
    _padding_0x004: [u32; 3],
    /// saturn_led_top_reg_4.
    pub rd_sdw_reg_en: RW<u32>,
    _padding_0x014: [u32; 1],
    /// saturn_led_top_reg_6.
    pub shut_down_num: RW<u32>,
    /// saturn_led_top_reg_7.
    pub deep_slep_num: RW<u32>,
    _padding_0x020: [u32; 3],
    /// saturn_led_top_reg_11.
    pub rdma_pclk_cg_en: RW<u32>,
    /// saturn_led_top_reg_12.
    pub pre_ly_pclk_cg_en: RW<u32>,
    /// saturn_led_top_reg_13.
    pub cmps_pclk_cg_en: RW<u32>,
    /// saturn_led_top_reg_14.
    pub scale_pclk_cg_en: RW<u32>,
    /// saturn_led_top_reg_15.
    pub postpipe_pclk_cg_en: RW<u32>,
    /// saturn_led_top_reg_16.
    pub wb_pclk_cg_en: RW<u32>,
    _padding_0x044: [u32; 5],
    /// saturn_led_top_reg_22.
    pub top_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_23.
    pub ctl_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_24.
    pub cmdlist_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_25.
    pub rdma_aclk_auto_en: RW<u32>,
    /// saturn_led_top_reg_26.
    pub pre_ly_mclk_auto_en: RW<u32>,
    /// saturn_led_top_reg_27.
    pub cmps_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_28.
    pub scale_mclk_auto_en: RW<u32>,
    /// saturn_led_top_reg_29.
    pub postpipe_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_30.
    pub tmg_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_31.
    pub wb_clk_auto_en: RW<u32>,
    /// saturn_led_top_reg_32.
    pub dma_mem_lp_en: RW<u32>,
    /// saturn_led_top_reg_33.
    pub postpipe_mem_lp_en: RW<u32>,
    /// saturn_led_top_reg_34.
    pub tmg_mem_lp_en: RW<u32>,
    _padding_0x08c: [u32; 1],
    /// saturn_led_top_reg_36.
    pub prepipe0_valid: RO<u32>,
    /// saturn_led_top_reg_37.
    pub cfg_se: RW<u32>,
    /// saturn_led_top_reg_38.
    pub force_update_en: RW<u32>,
    /// saturn_led_top_reg_39.
    pub force_update_pulse: RWNoModify<u32>,
    /// saturn_led_top_reg_40.
    pub force_update_en_se: RW<u32>,
    /// saturn_led_top_reg_41.
    pub force_update_pulse_se: RWNoModify<u32>,
    /// saturn_led_top_reg_42.
    pub icg_override: RW<u32>,
    /// saturn_led_top_reg_43.
    pub trigger: RWNoModify<u32>,
    /// saturn_led_top_reg_44.
    pub trigger2: RWNoModify<u32>,
    _padding_0x0b4: [u32; 195],
    /// Scene and command-list control.
    pub control: Control,
    _padding_0x494: [u32; 155],
    /// Display interrupt registers.
    pub interrupt: Interrupt,
}

/// DPU control registers.
#[repr(C)]
pub struct Control {
    /// cmdlist_rch_en.
    pub dpu_ctl_top_reg_0: [RW<u32>; 10],
    /// cmdlist_cmps_other_en, cmdlist_cmps_top_en.
    pub dpu_ctl_top_reg_10: [RW<u32>; 3],
    /// cmdlist_rdma_cfg_rdy, cmdlist_prepq_cfg_rdy, cmdlist_cmps_cfg_rdy, cmdlist_wb_cfg_rdy.
    pub dpu_ctl_top_reg_13: RWNoModify<u32>,
    /// wb0_sel_id.
    pub dpu_ctl_top_reg_14: RW<u32>,
    /// wb1_sel_id.
    pub dpu_ctl_top_reg_15: RW<u32>,
    /// reuse_rdma_act, reuse_prepq_act, reuse_scl_act0, reuse_cmps_act, reuse_wb_act, reuse_scl_act1.
    pub dpu_ctl_top_reg_16: RW<u32>,
    /// rch_conflict_ints, acad_timeout_ints, wb_timeout_ints, cmdlist_rdma_cfg_timeout_ints.
    pub dpu_ctl_top_reg_17: RWNoModify<u32>,
    /// rch_conflict_int_en, acad_timeout_int_en, wb_timeout_int_en, cmdlist_rdma_cfg_timeout_int_en.
    pub dpu_ctl_top_reg_18: RW<u32>,
    /// rch_conflict_int_raw, acad_timeout_int_raw, wb_timeout_int_raw, cmdlist_rdma_cfg_timeout_int_raw.
    pub dpu_ctl_top_reg_19: RO<u32>,
    /// wb_sel_secu, secu_cfg_icg_override, scl_sel_secu.
    pub dpu_ctl_top_reg_20: RW<u32>,
    /// pslverr_addr.
    pub dpu_ctl_top_reg_21: RO<u32>,
    /// cmdlist_ch_sw_event.
    pub dpu_ctl_top_reg_22: [RWNoModify<u32>; 13],
    /// ctl_rd_shadow.
    pub dpu_ctl_top_reg_35: RW<u32>,
    /// rch_start_cmps_y.
    pub dpu_ctl_top_reg_36: [RW<u32>; 10],
    /// wb0_slice_cnt, wb1_slice_cnt, wb_busy, acad_busy, dscw_busy.
    pub dpu_ctl_top_reg_46: RO<u32>,
    _padding_0x0bc: [u32; 2],
    /// nml_scl0_reuse_en.
    pub dpu_ctl_top_reg_49: RW<u32>,
    /// nml_scl1_reuse_en.
    pub dpu_ctl_top_reg_50: RW<u32>,
    /// nml_scl2_reuse_en.
    pub dpu_ctl_top_reg_51: RW<u32>,
    /// nml_scl3_reuse_en.
    pub dpu_ctl_top_reg_52: RW<u32>,
}

/// DPU interrupt registers.
#[repr(C)]
pub struct Interrupt {
    _padding_0x000: [u32; 3],
    /// cmb_frm_timing_vsync_int_msk.
    pub dpu_int_reg_3: RW<u32>,
    /// cmb_cmdlist_ch_enter_pend_int_msk.
    pub dpu_int_reg_4: RW<u32>,
    /// cmb_dma_dbg_int_msk.
    pub dpu_int_reg_5: RW<u32>,
    /// offl0_cfg_rdy_clr_int_msk.
    pub dpu_int_reg_6: RW<u32>,
    /// offl0_cmdlist_ch_enter_pend_int_msk.
    pub dpu_int_reg_7: RW<u32>,
    /// offl0_nml_dma_dbg_int_msk.
    pub dpu_int_reg_8: RW<u32>,
    _padding_0x024: [u32; 3],
    /// cmb_frm_timing_vsync_ints.
    pub dpu_int_reg_12: RW1C<u32>,
    /// cmb_cmdlist_ch_enter_pend_ints.
    pub dpu_int_reg_13: RWNoModify<u32>,
    /// cmb_dma_dbg_ints.
    pub dpu_int_reg_14: RWNoModify<u32>,
    /// offl0_cfg_rdy_clr_ints.
    pub dpu_int_reg_15: RW1C<u32>,
    /// off0_cmdlist_ch_enter_pend_ints.
    pub dpu_int_reg_16: RWNoModify<u32>,
    /// offl0_nml_dma_dbg_ints.
    pub dpu_int_reg_17: RWNoModify<u32>,
    _padding_0x048: [u32; 4],
    /// cmb_frm_timing_vsync_int_raw.
    pub dpu_int_reg_22: RO<u32>,
    /// cmb_cmdlist_ch_enter_pend_int_raw.
    pub dpu_int_reg_23: RO<u32>,
    /// cmb_dma_dbg_int_raw.
    pub dpu_int_reg_24: RO<u32>,
    /// cmb_arb_dec_paddr.
    pub dpu_int_reg_25: RO<u32>,
    /// offl0_cfg_rdy_clr_int_raw.
    pub dpu_int_reg_26: RO<u32>,
    /// off0_cmdlist_ch_enter_pend_int_raw.
    pub dpu_int_reg_27: RO<u32>,
    /// offl0_nml_dma_dbg_int_raw.
    pub dpu_int_reg_28: RO<u32>,
    /// offl0_arb_dec_paddr.
    pub dpu_int_reg_29: RO<u32>,
    _padding_0x078: [u32; 3],
    /// cmb_frm_timing_vsync_secu_int_msk.
    pub dpu_int_reg_33: RW<u32>,
    /// cmb_cmdlist_ch_enter_pend_secu_int_msk.
    pub dpu_int_reg_34: RW<u32>,
    /// cmb_dma_dbg_secu_int_msk.
    pub dpu_int_reg_35: RW<u32>,
    _padding_0x090: [u32; 3],
    /// cmb_frm_timing_vsync_secu_ints.
    pub dpu_int_reg_39: RWNoModify<u32>,
    /// cmb_cmdlist_ch_enter_pend_secu_ints.
    pub dpu_int_reg_40: RWNoModify<u32>,
    /// cmb_dma_dbg_secu_ints.
    pub dpu_int_reg_41: RWNoModify<u32>,
    _padding_0x0a8: [u32; 4],
    /// cmb_frm_timing_vsync_secu_int_raw.
    pub dpu_int_reg_46: RO<u32>,
    /// cmb_cmdlist_ch_enter_pend_secu_int_raw.
    pub dpu_int_reg_47: RO<u32>,
    /// cmb_dma_dbg_secu_int_raw.
    pub dpu_int_reg_48: RO<u32>,
    /// cmb_secu_arb_dec_paddr.
    pub dpu_int_reg_49: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, hardware_id), 0x000);
        assert_eq!(offset_of!(RegisterBlock, rd_sdw_reg_en), 0x010);
        assert_eq!(offset_of!(RegisterBlock, shut_down_num), 0x018);
        assert_eq!(offset_of!(RegisterBlock, deep_slep_num), 0x01c);
        assert_eq!(offset_of!(RegisterBlock, rdma_pclk_cg_en), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, pre_ly_pclk_cg_en), 0x030);
        assert_eq!(offset_of!(RegisterBlock, cmps_pclk_cg_en), 0x034);
        assert_eq!(offset_of!(RegisterBlock, scale_pclk_cg_en), 0x038);
        assert_eq!(offset_of!(RegisterBlock, postpipe_pclk_cg_en), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, wb_pclk_cg_en), 0x040);
        assert_eq!(offset_of!(RegisterBlock, top_clk_auto_en), 0x058);
        assert_eq!(offset_of!(RegisterBlock, ctl_clk_auto_en), 0x05c);
        assert_eq!(offset_of!(RegisterBlock, cmdlist_clk_auto_en), 0x060);
        assert_eq!(offset_of!(RegisterBlock, rdma_aclk_auto_en), 0x064);
        assert_eq!(offset_of!(RegisterBlock, pre_ly_mclk_auto_en), 0x068);
        assert_eq!(offset_of!(RegisterBlock, cmps_clk_auto_en), 0x06c);
        assert_eq!(offset_of!(RegisterBlock, scale_mclk_auto_en), 0x070);
        assert_eq!(offset_of!(RegisterBlock, postpipe_clk_auto_en), 0x074);
        assert_eq!(offset_of!(RegisterBlock, tmg_clk_auto_en), 0x078);
        assert_eq!(offset_of!(RegisterBlock, wb_clk_auto_en), 0x07c);
        assert_eq!(offset_of!(RegisterBlock, dma_mem_lp_en), 0x080);
        assert_eq!(offset_of!(RegisterBlock, postpipe_mem_lp_en), 0x084);
        assert_eq!(offset_of!(RegisterBlock, tmg_mem_lp_en), 0x088);
        assert_eq!(offset_of!(RegisterBlock, prepipe0_valid), 0x090);
        assert_eq!(offset_of!(RegisterBlock, cfg_se), 0x094);
        assert_eq!(offset_of!(RegisterBlock, force_update_en), 0x098);
        assert_eq!(offset_of!(RegisterBlock, force_update_pulse), 0x09c);
        assert_eq!(offset_of!(RegisterBlock, force_update_en_se), 0x0a0);
        assert_eq!(offset_of!(RegisterBlock, force_update_pulse_se), 0x0a4);
        assert_eq!(offset_of!(RegisterBlock, icg_override), 0x0a8);
        assert_eq!(offset_of!(RegisterBlock, trigger), 0x0ac);
        assert_eq!(offset_of!(RegisterBlock, trigger2), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, control), 0x3c0);
        assert_eq!(offset_of!(RegisterBlock, interrupt), 0x700);
        assert_eq!(size_of::<Control>(), 0xd4);
        assert_eq!(size_of::<Interrupt>(), 0xc8);
        assert_eq!(size_of::<RegisterBlock>(), 0x7c8);
    }
}
