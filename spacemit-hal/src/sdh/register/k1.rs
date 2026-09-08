//! K1/M1 SD, SDIO and eMMC host registers.

use crate::register::{RC, RWNoModify};
use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k1/k1_docs/k1_usermanual/10.Memory_%26_Storage.md
// The manual groups these registers as aligned 32-bit words.

/// K1/M1 SD, SDIO and eMMC host registers.
#[repr(C)]
pub struct RegisterBlock {
    /// SD_SYS_ADDR.
    pub sd_sys_addr: RW<u32>,
    /// SD_BLOCK_SIZE_CNT.
    pub sd_block_size_cnt: RW<u32>,
    /// SD_ARG.
    pub sd_arg: RW<u32>,
    /// SD_TRANSFER_MODE_CMD (mixed fields or command; no modify).
    pub sd_transfer_mode_cmd: RWNoModify<u32>,
    /// SD_RESP_0.
    pub sd_resp_0: RO<u32>,
    /// SD_RESP_1.
    pub sd_resp_1: RO<u32>,
    /// SD_RESP_2.
    pub sd_resp_2: RO<u32>,
    /// SD_RESP_3.
    pub sd_resp_3: RO<u32>,
    /// SD_BUFFER_DATA_PORT (mixed fields or command; no modify).
    pub sd_buffer_data_port: RWNoModify<u32>,
    /// SD_PRESENT_STATE_1.
    pub sd_present_state_1: RO<u32>,
    /// SD_HOST_CTRL (mixed fields or command; no modify).
    pub sd_host_ctrl: RWNoModify<u32>,
    /// SD_CLOCK_CTRL (mixed fields or command; no modify).
    pub sd_clock_ctrl: RWNoModify<u32>,
    /// SD_NORMAL_INT_STATUS (mixed fields or command; no modify).
    pub sd_normal_int_status: RWNoModify<u32>,
    /// SD_NORMAL_INT_STATUS_EN.
    pub sd_normal_int_status_en: RW<u32>,
    /// SD_NORMAL_INT_STATUS_INT_EN.
    pub sd_normal_int_status_int_en: RW<u32>,
    /// SD_AUTO_CMD12_ERROR_STATUS (mixed fields or command; no modify).
    pub sd_auto_cmd12_error_status: RWNoModify<u32>,
    /// SD_CAPABILITIES_1.
    pub sd_capabilities_1: RW<u32>,
    /// SD_CAPABILITIES_3.
    pub sd_capabilities_3: RO<u32>,
    /// SD_MAX_CURRENT_1.
    pub sd_max_current_1: RO<u32>,
    _padding_0x04c: [u32; 1],
    /// SD_FORCE_EVENT_AUTO_CMD12_ERROR.
    pub sd_force_event_auto_cmd12_error: WO<u32>,
    /// SD_ADMA_ERROR_STATUS.
    pub sd_adma_error_status: RW<u32>,
    /// ADMA SYSTEM ADDRESS REGISTER 1.
    pub adma_system_address_register_1: RW<u32>,
    /// SD_ADMA_SYS_ADDR_3.
    pub sd_adma_sys_addr_3: RW<u32>,
    /// PRESET_VALUE_FOR_INIT.
    pub preset_value_for_init: RO<u32>,
    /// PRESET_VALUE_FOR_HS.
    pub preset_value_for_hs: RO<u32>,
    /// PRESET_VALUE_FOR_SDR25.
    pub preset_value_for_sdr25: RO<u32>,
    /// PRESET_VALUE_FOR_SDR104.
    pub preset_value_for_sdr104: RO<u32>,
    _padding_0x070: [u32; 28],
    /// SHARED_BUS_CTRL.
    pub shared_bus_ctrl: RW<u32>,
    _padding_0x0e4: [u32; 6],
    /// SD_SLOT_INT_STATUS.
    pub sd_slot_int_status: RO<u32>,
    /// SDHC_VID_PID.
    pub sdhc_vid_pid: RO<u32>,
    /// SDHC_OP_CTRL.
    pub sdhc_op_ctrl: RW<u32>,
    /// SDHC_OP_EXT_REG.
    pub sdhc_op_ext: RW<u32>,
    /// SDHC_LEGACY_CTRL_REG (mixed fields or command; no modify).
    pub sdhc_legacy_ctrl: RWNoModify<u32>,
    /// SDHC_LEGACY_CEATA_REG.
    pub sdhc_legacy_ceata: RW<u32>,
    /// SDHC_MMC_CTRL_REG (mixed fields or command; no modify).
    pub sdhc_mmc_ctrl: RWNoModify<u32>,
    /// SDHC_RX_CFG_REG.
    pub sdhc_rx_cfg: RW<u32>,
    /// SDHC_TX_CFG_REG.
    pub sdhc_tx_cfg: RW<u32>,
    /// SDHC_HWTUNE_CFG_REG.
    pub sdhc_hwtune_cfg: RW<u32>,
    /// SDHC_HWTUNE_CFG2_REG.
    pub sdhc_hwtune_cfg2: RW<u32>,
    /// SDHC_ROUNDTRIP_TIMING_REG.
    pub sdhc_roundtrip_timing: RW<u32>,
    /// SDHC_GPIO_CFG_REG.
    pub sdhc_gpio_cfg: RO<u32>,
    /// SDHC_DLINE_CTRL_REG.
    pub sdhc_dline_ctrl: RW<u32>,
    /// SDHC_DLINE_CFG_REG.
    pub sdhc_dline_cfg: RW<u32>,
    _padding_0x138: [u32; 10],
    /// SDHC_PHY_CTRL_REG.
    pub sdhc_phy_ctrl: RW<u32>,
    /// SDHC_PHY_FUNC_REG.
    pub sdhc_phy_func: RW<u32>,
    /// SDHC_PHY_DLLCFG_REG.
    pub sdhc_phy_dllcfg: RW<u32>,
    /// SDHC_PHY_DLLCFG1_REG.
    pub sdhc_phy_dllcfg1: RW<u32>,
    /// SDHC_PHY_DLLSTS_REG.
    pub sdhc_phy_dllsts: RO<u32>,
    /// SDHC_PHY_DLLSTS1_REG.
    pub sdhc_phy_dllsts1: RO<u32>,
    /// SDHC_PHY_PADCFG_REG.
    pub sdhc_phy_padcfg: RW<u32>,
    /// SDHC_PHY_PADCFG1_REG.
    pub sdhc_phy_padcfg1: RW<u32>,
    /// SDHC_PHY_LBCTRL_REG (mixed fields or command; no modify).
    pub sdhc_phy_lbctrl: RWNoModify<u32>,
    /// SDHC_PHY_LBFUNC_REG.
    pub sdhc_phy_lbfunc: RW<u32>,
    /// SDHC_PHY_LBCNT_REG.
    pub sdhc_phy_lbcnt: RW<u32>,
    /// SDHC_PHY_LBSTS_REG (read has side effects).
    pub sdhc_phy_lbsts: RC<u32>,
    _padding_0x190: [u32; 24],
    /// CQE_CQBDCTRL_REG0.
    pub cqe_cqbdctrl_reg0: RW<u32>,
    /// CQE_CQBDCTRL_REG1.
    pub cqe_cqbdctrl_reg1: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, sd_sys_addr), 0x0);
        assert_eq!(offset_of!(RegisterBlock, sd_block_size_cnt), 0x4);
        assert_eq!(offset_of!(RegisterBlock, sd_arg), 0x8);
        assert_eq!(offset_of!(RegisterBlock, sd_transfer_mode_cmd), 0xc);
        assert_eq!(offset_of!(RegisterBlock, sd_resp_0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, sd_resp_1), 0x14);
        assert_eq!(offset_of!(RegisterBlock, sd_resp_2), 0x18);
        assert_eq!(offset_of!(RegisterBlock, sd_resp_3), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, sd_buffer_data_port), 0x20);
        assert_eq!(offset_of!(RegisterBlock, sd_present_state_1), 0x24);
        assert_eq!(offset_of!(RegisterBlock, sd_host_ctrl), 0x28);
        assert_eq!(offset_of!(RegisterBlock, sd_clock_ctrl), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, sd_normal_int_status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, sd_normal_int_status_en), 0x34);
        assert_eq!(offset_of!(RegisterBlock, sd_normal_int_status_int_en), 0x38);
        assert_eq!(offset_of!(RegisterBlock, sd_auto_cmd12_error_status), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, sd_capabilities_1), 0x40);
        assert_eq!(offset_of!(RegisterBlock, sd_capabilities_3), 0x44);
        assert_eq!(offset_of!(RegisterBlock, sd_max_current_1), 0x48);
        assert_eq!(
            offset_of!(RegisterBlock, sd_force_event_auto_cmd12_error),
            0x50
        );
        assert_eq!(offset_of!(RegisterBlock, sd_adma_error_status), 0x54);
        assert_eq!(
            offset_of!(RegisterBlock, adma_system_address_register_1),
            0x58
        );
        assert_eq!(offset_of!(RegisterBlock, sd_adma_sys_addr_3), 0x5c);
        assert_eq!(offset_of!(RegisterBlock, preset_value_for_init), 0x60);
        assert_eq!(offset_of!(RegisterBlock, preset_value_for_hs), 0x64);
        assert_eq!(offset_of!(RegisterBlock, preset_value_for_sdr25), 0x68);
        assert_eq!(offset_of!(RegisterBlock, preset_value_for_sdr104), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, shared_bus_ctrl), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, sd_slot_int_status), 0xfc);
        assert_eq!(offset_of!(RegisterBlock, sdhc_vid_pid), 0x100);
        assert_eq!(offset_of!(RegisterBlock, sdhc_op_ctrl), 0x104);
        assert_eq!(offset_of!(RegisterBlock, sdhc_op_ext), 0x108);
        assert_eq!(offset_of!(RegisterBlock, sdhc_legacy_ctrl), 0x10c);
        assert_eq!(offset_of!(RegisterBlock, sdhc_legacy_ceata), 0x110);
        assert_eq!(offset_of!(RegisterBlock, sdhc_mmc_ctrl), 0x114);
        assert_eq!(offset_of!(RegisterBlock, sdhc_rx_cfg), 0x118);
        assert_eq!(offset_of!(RegisterBlock, sdhc_tx_cfg), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, sdhc_hwtune_cfg), 0x120);
        assert_eq!(offset_of!(RegisterBlock, sdhc_hwtune_cfg2), 0x124);
        assert_eq!(offset_of!(RegisterBlock, sdhc_roundtrip_timing), 0x128);
        assert_eq!(offset_of!(RegisterBlock, sdhc_gpio_cfg), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, sdhc_dline_ctrl), 0x130);
        assert_eq!(offset_of!(RegisterBlock, sdhc_dline_cfg), 0x134);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_ctrl), 0x160);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_func), 0x164);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_dllcfg), 0x168);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_dllcfg1), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_dllsts), 0x170);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_dllsts1), 0x174);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_padcfg), 0x178);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_padcfg1), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_lbctrl), 0x180);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_lbfunc), 0x184);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_lbcnt), 0x188);
        assert_eq!(offset_of!(RegisterBlock, sdhc_phy_lbsts), 0x18c);
        assert_eq!(offset_of!(RegisterBlock, cqe_cqbdctrl_reg0), 0x1f0);
        assert_eq!(offset_of!(RegisterBlock, cqe_cqbdctrl_reg1), 0x1f4);
        assert_eq!(size_of::<RegisterBlock>(), 0x1f8);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
