use crate::gpio::RW1C;
use volatile_register::{RO, RW};

mod commons;
pub use commons::{ControlRegister, DataBuffer};

// Offsets and accesses: Linux i2c-k1.c and the vendor spacemit_i2c structure.
// https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-k1.c
// https://github.com/spacemit-com/uboot-2022.10/blob/1fa1ca64e9705a3650bcc7c21f6666949290830f/drivers/i2c/spacemit_i2c.c
// K3 declares the same interface through its spacemit,k1-i2c compatible fallback.
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// Only the first 0x20 bytes are modeled; FIFO registers at 0x20..0x37 are omitted.
// ICR contains configuration and commands: write deliberate complete values.
// ISR mixes read-only state and W1C events: acknowledge only selected event bits.

/// K1/M1 and K3 I2C registers for byte-mode transfers.
#[repr(C)]
pub struct RegisterBlock {
    /// Unit control and transfer commands (ICR).
    pub control: ControlRegister,
    /// Bus status and write-one-to-clear events (ISR).
    pub status: RW1C<u32>,
    /// Local slave address (ISAR).
    pub slave_address: RW<u32>,
    /// Transmit and receive byte buffer (IDBR).
    pub data_buffer: DataBuffer,
    /// SCL load counts (ILCR).
    pub load_count: RW<u32>,
    /// Clock wait counts (IWCR).
    pub wait_count: RW<u32>,
    /// Bus-reset cycles and SDA glitch control (IRCR).
    pub reset_cycle: RW<u32>,
    /// Physical SDA and SCL levels (IBMR).
    pub bus_monitor: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, status), 0x04);
        assert_eq!(offset_of!(RegisterBlock, slave_address), 0x08);
        assert_eq!(offset_of!(RegisterBlock, data_buffer), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, load_count), 0x10);
        assert_eq!(offset_of!(RegisterBlock, wait_count), 0x14);
        assert_eq!(offset_of!(RegisterBlock, reset_cycle), 0x18);
        assert_eq!(offset_of!(RegisterBlock, bus_monitor), 0x1c);
        assert_eq!(size_of::<RegisterBlock>(), 0x20);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn status_acknowledgement_writes_only_selected_events() {
        // SAFETY: All fields accept zero and this is exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        // Test memory records the write value; it does not emulate hardware W1C.
        // SAFETY: Seed mock status, then acknowledge only the selected TX event.
        unsafe {
            (&registers.status as *const RW1C<u32>)
                .cast_mut()
                .cast::<u32>()
                .write_volatile(0x01fd_c000);
            registers.status.clear(1 << 19);
        }
        assert_eq!(registers.status.read(), 1 << 19);
    }
}
