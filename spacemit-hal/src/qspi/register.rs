use crate::gpio::RW1C;
use volatile_register::{RO, RW, WO};

mod commons;
pub use commons::ModuleControl;

// K1 register offsets and access sequences: Linux spi-fsl-qspi.c and vendor QSPI.
// https://github.com/torvalds/linux/blob/master/drivers/spi/spi-fsl-qspi.c
// https://github.com/spacemit-com/uboot-2022.10/blob/1fa1ca64e9705a3650bcc7c21f6666949290830f/drivers/spi/k1x_qspi.c
// K3 declares the same register interface via its spacemit,k1-qspi fallback.
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// This exposes the IP-transfer subset, not the controller's entire MMIO window.
// IPCR, SPTRCLR, LUTKEY and LCKCR are write/command-only here to prohibit RMW.

/// K1/M1 and K3 QSPI registers used for IP transfers.
#[repr(C)]
pub struct RegisterBlock {
    /// Module configuration and FIFO-clear commands (MCR).
    pub module_control: ModuleControl,
    _reserved_0x004: [u32; 1],
    /// IP sequence launch command (IPCR).
    pub ip_command: WO<u32>,
    /// Serial-flash timing control (FLSHCR).
    pub flash_control: RW<u32>,
    _reserved_0x010: [u32; 5],
    /// SoC-specific control (SOCCR).
    pub soc_control: RW<u32>,
    _reserved_0x028: [u32; 54],
    /// Serial-flash command address (SFAR).
    pub flash_address: RW<u32>,
    /// Serial-flash address configuration (SFACR).
    pub flash_address_control: RW<u32>,
    /// Serial-flash sampling configuration (SMPR).
    pub sampling: RW<u32>,
    _reserved_0x10c: [u32; 1],
    /// Receive-buffer control (RBCT).
    pub receive_buffer_control: RW<u32>,
    _reserved_0x114: [u32; 15],
    /// Transmit-buffer status (TBSR).
    pub transmit_buffer_status: RO<u32>,
    /// Transmit FIFO push port (TBDR).
    pub transmit_buffer_data: WO<u32>,
    /// Transmit-buffer control (TBCT).
    pub transmit_buffer_control: RW<u32>,
    /// Controller activity status (SR).
    pub status: RO<u32>,
    /// Status flags with write-one-to-clear acknowledgements (FR).
    pub flags: RW1C<u32>,
    /// Interrupt and DMA request enables (RSER).
    pub interrupt_dma_enable: RW<u32>,
    _reserved_0x168: [u32; 1],
    /// Sequence-pointer clear command (SPTRCLR).
    pub pointer_clear: WO<u32>,
    _reserved_0x170: [u32; 4],
    /// Flash A1, A2, B1, and B2 end addresses (SFxxAD).
    pub flash_top_address: [RW<u32>; 4],
    _reserved_0x190: [u32; 28],
    /// Receive-buffer data words (RBDR).
    pub receive_buffer: [RO<u32>; 32],
    _reserved_0x280: [u32; 32],
    /// LUT lock/unlock key (LUTKEY).
    pub lut_key: WO<u32>,
    /// LUT lock/unlock command (LCKCR).
    pub lut_control: WO<u32>,
    _reserved_0x308: [u32; 2],
    /// Sixteen four-word instruction sequences (LUT).
    pub lut: [RW<u32>; 64],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, module_control), 0x000);
        assert_eq!(offset_of!(RegisterBlock, ip_command), 0x008);
        assert_eq!(offset_of!(RegisterBlock, flash_control), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, soc_control), 0x024);
        assert_eq!(offset_of!(RegisterBlock, flash_address), 0x100);
        assert_eq!(offset_of!(RegisterBlock, flash_address_control), 0x104);
        assert_eq!(offset_of!(RegisterBlock, sampling), 0x108);
        assert_eq!(offset_of!(RegisterBlock, receive_buffer_control), 0x110);
        assert_eq!(offset_of!(RegisterBlock, transmit_buffer_status), 0x150);
        assert_eq!(offset_of!(RegisterBlock, transmit_buffer_data), 0x154);
        assert_eq!(offset_of!(RegisterBlock, transmit_buffer_control), 0x158);
        assert_eq!(offset_of!(RegisterBlock, status), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, flags), 0x160);
        assert_eq!(offset_of!(RegisterBlock, interrupt_dma_enable), 0x164);
        assert_eq!(offset_of!(RegisterBlock, pointer_clear), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, flash_top_address), 0x180);
        assert_eq!(offset_of!(RegisterBlock, receive_buffer), 0x200);
        assert_eq!(offset_of!(RegisterBlock, lut_key), 0x300);
        assert_eq!(offset_of!(RegisterBlock, lut_control), 0x304);
        assert_eq!(offset_of!(RegisterBlock, lut), 0x310);
        assert_eq!(size_of::<RegisterBlock>(), 0x410);
        assert_eq!(align_of::<RegisterBlock>(), 4);

        // SAFETY: All fields accept zero and this is ordinary, owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        let base = &registers as *const _ as usize;
        for (index, register) in registers.flash_top_address.iter().enumerate() {
            assert_eq!(register as *const _ as usize - base, 0x180 + index * 4);
        }
        for (index, register) in registers.receive_buffer.iter().enumerate() {
            assert_eq!(register as *const _ as usize - base, 0x200 + index * 4);
        }
        for (index, register) in registers.lut.iter().enumerate() {
            assert_eq!(register as *const _ as usize - base, 0x310 + index * 4);
        }
    }

    #[test]
    fn acknowledgement_writes_only_the_selected_mask() {
        // SAFETY: All fields accept zero and the test exclusively owns this memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        // Test memory retains the bus write; it does not emulate hardware W1C.
        // SAFETY: This seeds mock status and then records a selective acknowledgement.
        unsafe {
            (&registers.flags as *const RW1C<u32>)
                .cast_mut()
                .cast::<u32>()
                .write_volatile(0xffff_ffff);
            registers.flags.clear(1 << 6);
        }
        assert_eq!(registers.flags.read(), 1 << 6);
    }
}
