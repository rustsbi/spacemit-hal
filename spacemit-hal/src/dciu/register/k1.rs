//! K1/M1 Dragon CPU configuration registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/8.CPU_System.md#825-dragon-ciu-registers-description
// Fabric debug exposes only the documented 0x170..0x17c range.

/// K1/M1 Dragon CPU configuration registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 5],
    /// CLUSTER0_CPU_SRAM_CTRL0_REG.
    pub cluster0_cpu_sram_ctrl0: RW<u32>,
    /// CLUSTER0_CPU_SRAM_CTRL1_REG.
    pub cluster0_cpu_sram_ctrl1: RW<u32>,
    _padding_0x01c: [u32; 14],
    /// CLUSTER1_CPU_SRAM_CTRL0_REG.
    pub cluster1_cpu_sram_ctrl0: RW<u32>,
    /// CLUSTER1_CPU_SRAM_CTRL1_REG.
    pub cluster1_cpu_sram_ctrl1: RW<u32>,
    _padding_0x05c: [u32; 11],
    /// CKG_CTRL_REG.
    pub ckg_ctrl: RW<u32>,
    _padding_0x08c: [u32; 1],
    /// CCI_DBG_CTRL_REG.
    pub cci_dbg_ctrl: RW<u32>,
    _padding_0x094: [u32; 1],
    /// CCI_INF_QOS_CTRL_REG.
    pub cci_inf_qos_ctrl: RW<u32>,
    /// CCI_SFRAM_CTL_REG.
    pub cci_sfram_ctl: RW<u32>,
    _padding_0x0a0: [u32; 20],
    /// X60_OUTER_COH_EN_CTRL_REG.
    pub x60_outer_coh_en_ctrl: RW<u32>,
    _padding_0x0f4: [u32; 9],
    /// FAB_TOM_RD_STS0_REG.
    pub fab_tom_rd_sts0: RO<u32>,
    /// FAB_TOM_RD_STS1_REG.
    pub fab_tom_rd_sts1: RO<u32>,
    /// AXI timeout monitors.
    pub fabric_timeout: [TimeoutMonitor; 5],
    /// Documented fabric debug words.
    pub fabric_debug: [RO<u32>; 4],
    _padding_0x180: [u32; 9],
    /// C0_INT_WAKEUP_MASK_REG.
    pub c0_int_wakeup_mask: RW<u32>,
    /// C1_INT_WAKEUP_MASK_REG.
    pub c1_int_wakeup_mask: RW<u32>,
    /// CPU_SYS_SW_RESET_REG.
    pub cpu_sys_sw_reset: RW<u32>,
    /// C0_L2_FLUSH_CTL_RESET_REG.
    pub c0_l2_flush_ctl_reset: RWNoModify<u32>,
    /// C1_L2_FLUSH_CTL_RESET_REG.
    pub c1_l2_flush_ctl_reset: RWNoModify<u32>,
    /// CPU_ACCESS_DDR_REMAP_EN_RESET_REG.
    pub cpu_access_ddr_remap_en_reset: RW<u32>,
    /// HAP_DM_CTL_REG.
    pub hap_dm_ctl: RWNoModify<u32>,
}

/// AXI fabric timeout monitor registers.
#[repr(C)]
pub struct TimeoutMonitor {
    /// Timeout configuration and acknowledgement.
    pub control: RWNoModify<u32>,
    /// transaction id.
    pub transaction_id: RO<u32>,
    /// status0.
    pub status0: RO<u32>,
    /// status1.
    pub status1: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(size_of::<TimeoutMonitor>(), 0x10);
        assert_eq!(offset_of!(TimeoutMonitor, status1), 0xc);
        assert_eq!(offset_of!(RegisterBlock, cluster0_cpu_sram_ctrl0), 0x014);
        assert_eq!(offset_of!(RegisterBlock, cluster0_cpu_sram_ctrl1), 0x018);
        assert_eq!(offset_of!(RegisterBlock, cluster1_cpu_sram_ctrl0), 0x054);
        assert_eq!(offset_of!(RegisterBlock, cluster1_cpu_sram_ctrl1), 0x058);
        assert_eq!(offset_of!(RegisterBlock, ckg_ctrl), 0x088);
        assert_eq!(offset_of!(RegisterBlock, cci_dbg_ctrl), 0x090);
        assert_eq!(offset_of!(RegisterBlock, cci_inf_qos_ctrl), 0x098);
        assert_eq!(offset_of!(RegisterBlock, cci_sfram_ctl), 0x09c);
        assert_eq!(offset_of!(RegisterBlock, x60_outer_coh_en_ctrl), 0x0f0);
        assert_eq!(offset_of!(RegisterBlock, fab_tom_rd_sts0), 0x118);
        assert_eq!(offset_of!(RegisterBlock, fab_tom_rd_sts1), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, fabric_timeout), 0x120);
        assert_eq!(offset_of!(RegisterBlock, fabric_debug), 0x170);
        assert_eq!(offset_of!(RegisterBlock, c0_int_wakeup_mask), 0x1a4);
        assert_eq!(offset_of!(RegisterBlock, c1_int_wakeup_mask), 0x1a8);
        assert_eq!(offset_of!(RegisterBlock, cpu_sys_sw_reset), 0x1ac);
        assert_eq!(offset_of!(RegisterBlock, c0_l2_flush_ctl_reset), 0x1b0);
        assert_eq!(offset_of!(RegisterBlock, c1_l2_flush_ctl_reset), 0x1b4);
        assert_eq!(
            offset_of!(RegisterBlock, cpu_access_ddr_remap_en_reset),
            0x1b8
        );
        assert_eq!(offset_of!(RegisterBlock, hap_dm_ctl), 0x1bc);
        assert_eq!(size_of::<RegisterBlock>(), 0x1c0);
    }
}
