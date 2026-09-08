//! K3 SD, SDIO and eMMC host registers.

use crate::register::{RC, RW1C, RWNoModify};
use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/09_memory_storage.md
// Section 9.6.3: aligned 32-bit register groups, including CQE.

/// K3 SD, SDIO and eMMC host registers.
#[repr(C)]
pub struct RegisterBlock {
    /// SYSTEM ADDRESS.
    pub sd_sys_addr: RW<u32>,
    /// BLOCK SIZE.
    pub sd_block_size_cnt: RW<u32>,
    /// ARGUMENT.
    pub sd_arg: RW<u32>,
    /// TRANSFER MODE AND COMMAND (mixed fields or command; no modify).
    pub sd_transfer_mode_cmd: RWNoModify<u32>,
    /// RESPONSE REGISTER 0.
    pub sd_resp_0: RO<u32>,
    /// RESPONSE REGISTER 1.
    pub sd_resp_1: RO<u32>,
    /// RESPONSE REGISTER 2.
    pub sd_resp_2: RO<u32>,
    /// RESPONSE REGISTER 3.
    pub sd_resp_3: RO<u32>,
    /// BUFFER DATA PORT 01 (mixed fields or command; no modify).
    pub sd_buffer_data_port: RWNoModify<u32>,
    /// PRESENT STATE REGISTER 1.
    pub sd_present_state_1: RO<u32>,
    /// HOST CONTROL (mixed fields or command; no modify).
    pub sd_host_ctrl: RWNoModify<u32>,
    /// CLOCK CONTROL (mixed fields or command; no modify).
    pub sd_clock_ctrl: RWNoModify<u32>,
    /// NORMAL INTERRUPT STATUS (mixed fields or command; no modify).
    pub sd_normal_int_status: RWNoModify<u32>,
    /// NORMAL INTERRUPT STATUS ENABLE.
    pub sd_normal_int_status_en: RW<u32>,
    /// NORMAL INTERRUPT STATUS INTERRUPT ENABLE.
    pub sd_normal_int_status_int_en: RW<u32>,
    /// AUTO CMD12 ERROR STATUS (mixed fields or command; no modify).
    pub sd_auto_cmd12_error_status: RWNoModify<u32>,
    /// CAPABILITIES REGISTER 1.
    pub sd_capabilities_1: RW<u32>,
    /// CAPABILITIES REGISTER 3.
    pub sd_capabilities_3: RO<u32>,
    /// MAXIMUM CURRENT REGISTER 1.
    pub sd_max_current_1: RO<u32>,
    _padding_0x04c: [u32; 1],
    /// FORCE EVENT AUTO CMD12 ERROR.
    pub sd_force_event_auto_cmd12_error: WO<u32>,
    /// ADMA ERROR STATUS.
    pub sd_adma_error_status: RW<u32>,
    /// ADMA SYSTEM ADDRESS REGISTER 1.
    pub adma_system_address_register_1: RW<u32>,
    /// ADMA SYSTEM ADDRESS REGISTER 3.
    pub sd_adma_sys_addr_3: RW<u32>,
    /// PRESET VALUE REGISTER FOR INITIALIZATION.
    pub preset_value_for_init: RO<u32>,
    /// PRESET VALUE REGISTER FOR HIGH SPEED.
    pub preset_value_for_hs: RO<u32>,
    /// PRESET VALUE REGISTER FOR SDR25.
    pub preset_value_for_sdr25: RO<u32>,
    /// PRESET VALUE REGISTER FOR SDR104.
    pub preset_value_for_sdr104: RO<u32>,
    _padding_0x070: [u32; 28],
    /// SHARED BUS CONTROL.
    pub shared_bus_ctrl: RW<u32>,
    _padding_0x0e4: [u32; 6],
    /// SLOT INTERRUPT STATUS.
    pub sd_slot_int_status: RO<u32>,
    /// SD HOST CTRL VENDOR ID/PROJECT ID/VERSION ID.
    pub sdhc_vid_pid: RO<u32>,
    /// SDHC OPEARTION CONTROL REGISTER (CLOCK AND BURST SIZE SETUP REGISTER).
    pub sdhc_op_ctrl: RW<u32>,
    /// SDHC OPERATION EXTEND CTRL.
    pub sdhc_op_ext: RW<u32>,
    /// SDHC LEGACY CTRL PARAMETERS (mixed fields or command; no modify).
    pub sdhc_legacy_ctrl: RWNoModify<u32>,
    /// SDHC LEGACY CTRL FOR CEATA DEVICE.
    pub sdhc_legacy_ceata: RW<u32>,
    /// SDHC MMC DEVICE CTRL (mixed fields or command; no modify).
    pub sdhc_mmc_ctrl: RWNoModify<u32>,
    /// SDHC RX CONFIGURATION.
    pub sdhc_rx_cfg: RW<u32>,
    /// SDHC TX CONFIGURATION.
    pub sdhc_tx_cfg: RW<u32>,
    /// SDHC HW TUNING CONFIGURATION.
    pub sdhc_hwtune_cfg: RW<u32>,
    /// SDHC HW TUNING CONFIGURATION2.
    pub sdhc_hwtune_cfg2: RW<u32>,
    /// SDHC ROUND TRIP(TRANSIMIT TO RECEIVE) TIMING PARAM REGSITER.
    pub sdhc_roundtrip_timing: RW<u32>,
    /// SDHC GPIO CFG.
    pub sdhc_gpio_cfg: RO<u32>,
    /// SDHC DELAYLINE CONTROL.
    pub sdhc_dline_ctrl: RW<u32>,
    /// SDHC DELAYLINE CFG.
    pub sdhc_dline_cfg: RW<u32>,
    _padding_0x138: [u32; 10],
    /// SDHC PHY CONTROL.
    pub sdhc_phy_ctrl: RW<u32>,
    /// SDHC PHY FUNCTION CONFIGURATION.
    pub sdhc_phy_func: RW<u32>,
    /// SDHC PHY DLL CONFIGURATION.
    pub sdhc_phy_dllcfg: RW<u32>,
    /// SDHC PHY DLL CONFIGURATION1.
    pub sdhc_phy_dllcfg1: RW<u32>,
    /// SDHC PHY DLL STATUS & RESERVED CONFIGURATION.
    pub sdhc_phy_dllsts: RO<u32>,
    /// SDHC PHY DLL STATUS1.
    pub sdhc_phy_dllsts1: RO<u32>,
    /// SDHC PHY PAD CONFIGURATION.
    pub sdhc_phy_padcfg: RW<u32>,
    /// SDHC PHY PAD CONFIGURATION1.
    pub sdhc_phy_padcfg1: RW<u32>,
    /// SDHC PHY LOOPBACK CONTROL (mixed fields or command; no modify).
    pub sdhc_phy_lbctrl: RWNoModify<u32>,
    /// SDHC PHY LOOPBACK FUNCTION CONFIGURATION.
    pub sdhc_phy_lbfunc: RW<u32>,
    /// SDHC PHY LOOPBACK COMPARISON COUNT.
    pub sdhc_phy_lbcnt: RW<u32>,
    /// SDHC PHY LOOPBACK ERROR STATUS (read has side effects).
    pub sdhc_phy_lbsts: RC<u32>,
    /// SDHC PHY LOOPBACK DATA PATTERN CONFIGURATION (mixed fields or command; no modify).
    pub phy_loopback_pattern: RWNoModify<u32>,
    _padding_0x194: [u32; 23],
    /// SDHC COMMAND QUEUE BUS DEBUG CONTROL.
    pub cqe_cqbdctrl_reg0: RW<u32>,
    /// SDHC COMMAND QUEUE DEBUG INFORMATION CONTENT.
    pub cqe_cqbdctrl_reg1: RO<u32>,
    _padding_0x1f8: [u32; 2],
    /// SDHC COMMAND QUEUE VERSION.
    pub cqe_version: RO<u32>,
    /// SDHC COMMAND QUEUE CAPABILITIES.
    pub cqe_capabilities: RO<u32>,
    /// SDHC COMMAND QUEUE CONFIG.
    pub cqe_configuration: RW<u32>,
    /// SDHC COMMAND QUEUE CONTROL (mixed fields or command; no modify).
    pub cqe_control: RWNoModify<u32>,
    /// SDHC COMMAND QUEUE INTERRUPT STATUS.
    pub cqe_interrupt_status: RW1C<u32>,
    /// SDHC COMMAND QUEUE INTERRUPT ENABLE.
    pub cqe_interrupt_enable: RW<u32>,
    /// SDHC COMMAND QUEUE INTERRUPT SIGNAL ENABLE.
    pub cqe_signal_enable: RW<u32>,
    /// SDHC COMMAND QUEUE INTERRUPT COALESCING (mixed fields or command; no modify).
    pub cqe_interrupt_coalescing: RWNoModify<u32>,
    /// SDHC COMMAND QUEUE TASK DESCRIPTOR LIST BASE ADDRESS.
    pub cqe_descriptor_base: RW<u32>,
    /// SDHC COMMAND QUEUE TASK DESCRIPTOR LIST UPPER BASE ADDRESS.
    pub cqe_descriptor_base_high: RW<u32>,
    /// SDHC COMMAND QUEUE TASK DOORBELL (mixed fields or command; no modify).
    pub cqe_doorbell: RWNoModify<u32>,
    /// SDHC COMMAND QUEUE TASK COMPLETE NOTIFICATION.
    pub cqe_task_complete: RW1C<u32>,
    /// SDHC COMMAND QUEUE DEVICE QUEUE STATUS.
    pub cqe_queue_status: RO<u32>,
    /// SDHC COMMAND QUEUE DEVICE PENDING TASKS.
    pub cqe_pending_tasks: RO<u32>,
    /// SDHC COMMAND QUEUE TASK CLEAR (mixed fields or command; no modify).
    pub cqe_task_clear: RWNoModify<u32>,
    _padding_0x23c: [u32; 1],
    /// SDHC COMMAND QUEUE SEND STATUS CONFIGURATION REGISTERS1.
    pub cqe_send_status_config1: RW<u32>,
    /// SDHC COMMAND QUEUE SEND STATUS CONFIGURATION REGISTERS2.
    pub cqe_send_status_config2: RW<u32>,
    /// SDHC COMMAND QUEUE COMMAND RESPONSE FOR DIRECT-COMMAND TASK.
    pub cqe_direct_command_response: RO<u32>,
    _padding_0x24c: [u32; 1],
    /// SDHC COMMAND QUEUE RESPONSE MODE ERROR MASK.
    pub cqe_response_error_mask: RW<u32>,
    /// SDHC COMMAND QUEUE TASK ERROR INFORMATION.
    pub cqe_task_error_info: RO<u32>,
    /// SDHC COMMAND QUEUE COMMAND RESPONSE INDEX.
    pub cqe_response_index: RO<u32>,
    /// SDHC COMMAND QUEUE COMMAND RESPONSE ARGUMENT.
    pub cqe_response_argument: RO<u32>,
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
        assert_eq!(offset_of!(RegisterBlock, phy_loopback_pattern), 0x190);
        assert_eq!(offset_of!(RegisterBlock, cqe_cqbdctrl_reg0), 0x1f0);
        assert_eq!(offset_of!(RegisterBlock, cqe_cqbdctrl_reg1), 0x1f4);
        assert_eq!(offset_of!(RegisterBlock, cqe_version), 0x200);
        assert_eq!(offset_of!(RegisterBlock, cqe_capabilities), 0x204);
        assert_eq!(offset_of!(RegisterBlock, cqe_configuration), 0x208);
        assert_eq!(offset_of!(RegisterBlock, cqe_control), 0x20c);
        assert_eq!(offset_of!(RegisterBlock, cqe_interrupt_status), 0x210);
        assert_eq!(offset_of!(RegisterBlock, cqe_interrupt_enable), 0x214);
        assert_eq!(offset_of!(RegisterBlock, cqe_signal_enable), 0x218);
        assert_eq!(offset_of!(RegisterBlock, cqe_interrupt_coalescing), 0x21c);
        assert_eq!(offset_of!(RegisterBlock, cqe_descriptor_base), 0x220);
        assert_eq!(offset_of!(RegisterBlock, cqe_descriptor_base_high), 0x224);
        assert_eq!(offset_of!(RegisterBlock, cqe_doorbell), 0x228);
        assert_eq!(offset_of!(RegisterBlock, cqe_task_complete), 0x22c);
        assert_eq!(offset_of!(RegisterBlock, cqe_queue_status), 0x230);
        assert_eq!(offset_of!(RegisterBlock, cqe_pending_tasks), 0x234);
        assert_eq!(offset_of!(RegisterBlock, cqe_task_clear), 0x238);
        assert_eq!(offset_of!(RegisterBlock, cqe_send_status_config1), 0x240);
        assert_eq!(offset_of!(RegisterBlock, cqe_send_status_config2), 0x244);
        assert_eq!(
            offset_of!(RegisterBlock, cqe_direct_command_response),
            0x248
        );
        assert_eq!(offset_of!(RegisterBlock, cqe_response_error_mask), 0x250);
        assert_eq!(offset_of!(RegisterBlock, cqe_task_error_info), 0x254);
        assert_eq!(offset_of!(RegisterBlock, cqe_response_index), 0x258);
        assert_eq!(offset_of!(RegisterBlock, cqe_response_argument), 0x25c);
        assert_eq!(size_of::<RegisterBlock>(), 0x260);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
