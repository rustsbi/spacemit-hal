//! K3 MIPI DSI registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/12_display.md
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/gpu/drm/spacemit/dsi/spacemit_dsi_drv.c

/// K3 MIPI DSI registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DSI_CTRL_0.
    pub dsi_ctrl_0: RWNoModify<u32>,
    /// DSI_CTRL_1.
    pub dsi_ctrl_1: RW<u32>,
    /// DSI_IRQ_ST1.
    pub dsi_irq_st1: RWNoModify<u32>,
    /// DSI_IRQ_MASK1.
    pub dsi_irq_mask1: RW<u32>,
    /// DSI_IRQ_ST.
    pub dsi_irq_st: RW1C<u32>,
    /// DSI_IRQ_MASK.
    pub dsi_irq_mask: RW<u32>,
    /// DSI_FPGA_PHY_CTRL_0.
    pub dsi_fpga_phy_ctrl_0: RW<u32>,
    /// DSI_FPGA_PHY_CTRL_1.
    pub dsi_fpga_phy_ctrl_1: RW<u32>,
    /// DSI_CPU_CMD_0.
    pub dsi_cpu_cmd_0: RWNoModify<u32>,
    /// DSI_CPU_CMD_1.
    pub dsi_cpu_cmd_1: RW<u32>,
    _padding_0x028: [u32; 1],
    /// DSI_CPU_CMD_3.
    pub dsi_cpu_cmd_3: RWNoModify<u32>,
    /// DSI_CPU_WDAT.
    pub dsi_cpu_wdat: RW<u32>,
    /// DSI_CPU_STATUS_0.
    pub dsi_cpu_status_0: RWNoModify<u32>,
    /// DSI_CPU_STATUS_1.
    pub dsi_cpu_status_1: RO<u32>,
    /// DSI_CPU_STATUS_2.
    pub dsi_cpu_status_2: RO<u32>,
    /// DSI_CPU_STATUS_3.
    pub dsi_cpu_status_3: RO<u32>,
    /// DSI_CPU_STATUS_4.
    pub dsi_cpu_status_4: RO<u32>,
    _padding_0x048: [u32; 1],
    /// DSI_CPN_STATUS_1.
    pub dsi_cpn_status_1: RO<u32>,
    /// DSI_CPN_CMD.
    pub dsi_cpn_cmd: RW<u32>,
    /// DSI_CPN_CTRL_0.
    pub dsi_cpn_ctrl_0: RW<u32>,
    /// DSI_CPN_CTRL_1.
    pub dsi_cpn_ctrl_1: RW<u32>,
    /// DSI_CPN_STATUS_0.
    pub dsi_cpn_status_0: RO<u32>,
    /// DSI_RX_PKT_ST_0.
    pub dsi_rx_pkt_st_0: RWNoModify<u32>,
    /// DSI_RX_PKT_HDR_0.
    pub dsi_rx_pkt_hdr_0: RW<u32>,
    /// DSI_RX_PKT_ST_1.
    pub dsi_rx_pkt_st_1: RWNoModify<u32>,
    /// DSI_RX_PKT_HDR_1.
    pub dsi_rx_pkt_hdr_1: RW<u32>,
    /// DSI_RX_PKT_CTRL.
    pub dsi_rx_pkt_ctrl: RWNoModify<u32>,
    /// DSI_RX_PKT_CTRL_1.
    pub dsi_rx_pkt_ctrl_1: RWNoModify<u32>,
    /// DSI_RX_PKT_ST_2.
    pub dsi_rx_pkt_st_2: RWNoModify<u32>,
    /// DSI_RX_PKT_HDR_2.
    pub dsi_rx_pkt_hdr_2: RW<u32>,
    _padding_0x080: [u32; 1],
    /// DSI_LCD_BDG_CTRL0.
    pub dsi_lcd_bdg_ctrl0: RW<u32>,
    /// DSI_LCD_BDG_CTRL1.
    pub dsi_lcd_bdg_ctrl1: RW<u32>,
    _padding_0x08c: [u32; 13],
    /// PLL_CTRL_REG0.
    pub pll_ctrl_reg0: RW<u32>,
    /// PLL_CTRL_REG1.
    pub pll_ctrl_reg1: RW<u32>,
    /// PLL_CTRL_REG2.
    pub pll_ctrl_reg2: RW<u32>,
    /// PLL_CTRL_REG3.
    pub pll_ctrl_reg3: RW<u32>,
    _padding_0x0d0: [u32; 5],
    /// DSI_TX_TIMER.
    pub dsi_tx_timer: RW<u32>,
    /// DSI_RX_TIMER.
    pub dsi_rx_timer: RW<u32>,
    /// DSI_TURN_TIMER.
    pub dsi_turn_timer: RW<u32>,
    _padding_0x0f0: [u32; 4],
    /// DSI_VPN_CTRL_0.
    pub dsi_vpn_ctrl_0: RW<u32>,
    /// DSI_VPN_CTRL_1.
    pub dsi_vpn_ctrl_1: RW<u32>,
    _padding_0x108: [u32; 2],
    /// DSI_VPN_TIMING_0.
    pub dsi_vpn_timing_0: RW<u32>,
    /// DSI_VPN_TIMING_1.
    pub dsi_vpn_timing_1: RW<u32>,
    /// DSI_VPN_TIMING_2.
    pub dsi_vpn_timing_2: RW<u32>,
    /// DSI_VPN_TIMING_3.
    pub dsi_vpn_timing_3: RW<u32>,
    /// DSI_VPN_WC_0.
    pub dsi_vpn_wc_0: RW<u32>,
    /// DSI_VPN_WC_1.
    pub dsi_vpn_wc_1: RW<u32>,
    /// DSI_VPN_WC_2.
    pub dsi_vpn_wc_2: RW<u32>,
    _padding_0x12c: [u32; 1],
    /// DSI_VPN_SLOT_CNT_0.
    pub dsi_vpn_slot_cnt_0: RW<u32>,
    /// DSI_VPN_SLOT_CNT_1.
    pub dsi_vpn_slot_cnt_1: RW<u32>,
    /// DSI_VPN_SYNC_CODE.
    pub dsi_vpn_sync_code: RW<u32>,
    _padding_0x13c: [u32; 1],
    /// DSI_VPN_STATUS_0.
    pub dsi_vpn_status_0: RO<u32>,
    /// DSI_VPN_STATUS_1.
    pub dsi_vpn_status_1: RO<u32>,
    /// DSI_VPN_STATUS_2.
    pub dsi_vpn_status_2: RO<u32>,
    /// DSI_VPN_STATUS_3.
    pub dsi_vpn_status_3: RO<u32>,
    /// DSI_VPN_STATUS_4.
    pub dsi_vpn_status_4: RO<u32>,
    /// DSI_VPN_STATUS_5.
    pub dsi_vpn_status_5: RO<u32>,
    _padding_0x158: [u32; 10],
    /// DSI_PHY_CTRL_0.
    pub dsi_phy_ctrl_0: RW<u32>,
    /// DSI_PHY_CTRL_1.
    pub dsi_phy_ctrl_1: RW<u32>,
    /// DSI_PHY_CTRL_2.
    pub dsi_phy_ctrl_2: RW<u32>,
    /// DSI_PHY_CTRL_3.
    pub dsi_phy_ctrl_3: RW<u32>,
    /// DSI_PHY_STATUS_0.
    pub dsi_phy_status_0: RWNoModify<u32>,
    /// DSI_PHY_STATUS_1.
    pub dsi_phy_status_1: RO<u32>,
    /// DSI_PHY_LPRX_0.
    pub dsi_phy_lprx_0: RO<u32>,
    /// DSI_PHY_LPRX_1.
    pub dsi_phy_lprx_1: RO<u32>,
    /// DSI_PHY_LPTX_0.
    pub dsi_phy_lptx_0: RO<u32>,
    /// DSI_PHY_LPTX_1.
    pub dsi_phy_lptx_1: RO<u32>,
    /// DSI_PHY_LPTX_2.
    pub dsi_phy_lptx_2: RO<u32>,
    /// DSI_PHY_STATUS_2.
    pub dsi_phy_status_2: RO<u32>,
    _padding_0x1b0: [u32; 4],
    /// DSI_PHY_TIME_0.
    pub dsi_phy_time_0: RW<u32>,
    /// DSI_PHY_TIME_1.
    pub dsi_phy_time_1: RW<u32>,
    /// DSI_PHY_TIME_2.
    pub dsi_phy_time_2: RW<u32>,
    /// DSI_PHY_TIME_3.
    pub dsi_phy_time_3: RW<u32>,
    /// DSI_PHY_CODE_0.
    pub dsi_phy_code_0: RW<u32>,
    /// DSI_PHY_CODE_1.
    pub dsi_phy_code_1: RWNoModify<u32>,
    _padding_0x1d8: [u32; 2],
    /// DSI_PHY_ANA_PWR_CTRL.
    pub dsi_phy_ana_pwr_ctrl: RW<u32>,
    /// DSI_PHY_ANA_CTRL0.
    pub dsi_phy_ana_ctrl0: RW<u32>,
    /// DSI_PHY_ANA_CTRL1.
    pub dsi_phy_ana_ctrl1: RW<u32>,
    /// DSI_PHY_DEBUG.
    pub dsi_phy_debug: RW<u32>,
    /// DSI_PHY_TIME_8.
    pub dsi_phy_time_8: RW<u32>,
    _padding_0x1f4: [u32; 3],
    /// DYN_PLL_CTRL_DSI.
    pub dyn_pll_ctrl_dsi: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, dsi_ctrl_0), 0x000);
        assert_eq!(offset_of!(RegisterBlock, dsi_ctrl_1), 0x004);
        assert_eq!(offset_of!(RegisterBlock, dsi_irq_st1), 0x008);
        assert_eq!(offset_of!(RegisterBlock, dsi_irq_mask1), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, dsi_irq_st), 0x010);
        assert_eq!(offset_of!(RegisterBlock, dsi_irq_mask), 0x014);
        assert_eq!(offset_of!(RegisterBlock, dsi_fpga_phy_ctrl_0), 0x018);
        assert_eq!(offset_of!(RegisterBlock, dsi_fpga_phy_ctrl_1), 0x01c);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_cmd_0), 0x020);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_cmd_1), 0x024);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_cmd_3), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_wdat), 0x030);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_status_0), 0x034);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_status_1), 0x038);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_status_2), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_status_3), 0x040);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpu_status_4), 0x044);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpn_status_1), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpn_cmd), 0x050);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpn_ctrl_0), 0x054);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpn_ctrl_1), 0x058);
        assert_eq!(offset_of!(RegisterBlock, dsi_cpn_status_0), 0x05c);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_st_0), 0x060);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_hdr_0), 0x064);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_st_1), 0x068);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_hdr_1), 0x06c);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_ctrl), 0x070);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_ctrl_1), 0x074);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_st_2), 0x078);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_pkt_hdr_2), 0x07c);
        assert_eq!(offset_of!(RegisterBlock, dsi_lcd_bdg_ctrl0), 0x084);
        assert_eq!(offset_of!(RegisterBlock, dsi_lcd_bdg_ctrl1), 0x088);
        assert_eq!(offset_of!(RegisterBlock, pll_ctrl_reg0), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, pll_ctrl_reg1), 0x0c4);
        assert_eq!(offset_of!(RegisterBlock, pll_ctrl_reg2), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, pll_ctrl_reg3), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, dsi_tx_timer), 0x0e4);
        assert_eq!(offset_of!(RegisterBlock, dsi_rx_timer), 0x0e8);
        assert_eq!(offset_of!(RegisterBlock, dsi_turn_timer), 0x0ec);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_ctrl_0), 0x100);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_ctrl_1), 0x104);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_timing_0), 0x110);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_timing_1), 0x114);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_timing_2), 0x118);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_timing_3), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_wc_0), 0x120);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_wc_1), 0x124);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_wc_2), 0x128);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_slot_cnt_0), 0x130);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_slot_cnt_1), 0x134);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_sync_code), 0x138);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_0), 0x140);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_1), 0x144);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_2), 0x148);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_3), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_4), 0x150);
        assert_eq!(offset_of!(RegisterBlock, dsi_vpn_status_5), 0x154);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ctrl_0), 0x180);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ctrl_1), 0x184);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ctrl_2), 0x188);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ctrl_3), 0x18c);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_status_0), 0x190);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_status_1), 0x194);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_lprx_0), 0x198);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_lprx_1), 0x19c);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_lptx_0), 0x1a0);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_lptx_1), 0x1a4);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_lptx_2), 0x1a8);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_status_2), 0x1ac);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_time_0), 0x1c0);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_time_1), 0x1c4);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_time_2), 0x1c8);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_time_3), 0x1cc);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_code_0), 0x1d0);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_code_1), 0x1d4);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ana_pwr_ctrl), 0x1e0);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ana_ctrl0), 0x1e4);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_ana_ctrl1), 0x1e8);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_debug), 0x1ec);
        assert_eq!(offset_of!(RegisterBlock, dsi_phy_time_8), 0x1f0);
        assert_eq!(offset_of!(RegisterBlock, dyn_pll_ctrl_dsi), 0x200);
        assert_eq!(size_of::<RegisterBlock>(), 0x204);
    }
}
