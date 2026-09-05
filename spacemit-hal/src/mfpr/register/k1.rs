//! K1 multi-function pad register layout.

use core::ops::Index;
use volatile_register::RW;

// Pin offsets: spacemit_k1_pin_to_offset in Linux pinctrl-k1.c.
// https://github.com/torvalds/linux/blob/master/drivers/pinctrl/spacemit/pinctrl-k1.c
// GPIO0..127 have gaps and reordered groups; this is not a pinctrl driver.
// Bit 6 clears edge detection; writes must account for that side effect.
// Offsets 0x228/0x22c are GPIO118/119, not the dedicated PWR_SCL/PWR_SDA pads.

/// K1 multi-function pad registers.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO0 through GPIO127 pad configuration, indexed by GPIO number.
    pub gpio: GpioRegisters,
    _reserved_0x250: [u32; 876],
}

/// GPIO pad registers with GPIO-number indexing over K1's sparse layout.
#[repr(C)]
pub struct GpioRegisters {
    _reserved_0x000: [u32; 1],
    gpio0_85: [RW<u32>; 86],
    _reserved_0x15c: [u32; 3],
    gpio101_98: [RW<u32>; 4],
    gpio103_102: [RW<u32>; 2],
    _reserved_0x180: [u32; 14],
    gpio104_110: [RW<u32>; 7],
    gpio93_97: [RW<u32>; 5],
    _reserved_0x1e8: [u32; 1],
    gpio86_92: [RW<u32>; 7],
    _reserved_0x208: [u32; 1],
    gpio111_127: [RW<u32>; 17],
}

impl Index<usize> for GpioRegisters {
    type Output = RW<u32>;

    #[inline]
    fn index(&self, pin: usize) -> &Self::Output {
        match pin {
            0..=85 => &self.gpio0_85[pin],
            86..=92 => &self.gpio86_92[pin - 86],
            93..=97 => &self.gpio93_97[pin - 93],
            98..=101 => &self.gpio101_98[101 - pin],
            102..=103 => &self.gpio103_102[103 - pin],
            104..=110 => &self.gpio104_110[pin - 104],
            111..=127 => &self.gpio111_127[pin - 111],
            _ => panic!("GPIO index out of bounds: {pin} >= 128"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, gpio), 0x000);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);

        assert_eq!(offset_of!(GpioRegisters, gpio0_85), 0x004);
        assert_eq!(offset_of!(GpioRegisters, gpio101_98), 0x168);
        assert_eq!(offset_of!(GpioRegisters, gpio103_102), 0x178);
        assert_eq!(offset_of!(GpioRegisters, gpio104_110), 0x1b8);
        assert_eq!(offset_of!(GpioRegisters, gpio93_97), 0x1d4);
        assert_eq!(offset_of!(GpioRegisters, gpio86_92), 0x1ec);
        assert_eq!(offset_of!(GpioRegisters, gpio111_127), 0x20c);
        assert_eq!(size_of::<GpioRegisters>(), 0x250);
        assert_eq!(align_of::<GpioRegisters>(), 4);
    }

    #[test]
    fn gpio_number_to_register_offset() {
        // Byte offsets from Linux's spacemit_k1_pin_to_offset, by GPIO number.
        let offsets: [usize; 128] = [
            0x004, 0x008, 0x00c, 0x010, 0x014, 0x018, 0x01c, 0x020, 0x024, 0x028, 0x02c, 0x030,
            0x034, 0x038, 0x03c, 0x040, 0x044, 0x048, 0x04c, 0x050, 0x054, 0x058, 0x05c, 0x060,
            0x064, 0x068, 0x06c, 0x070, 0x074, 0x078, 0x07c, 0x080, 0x084, 0x088, 0x08c, 0x090,
            0x094, 0x098, 0x09c, 0x0a0, 0x0a4, 0x0a8, 0x0ac, 0x0b0, 0x0b4, 0x0b8, 0x0bc, 0x0c0,
            0x0c4, 0x0c8, 0x0cc, 0x0d0, 0x0d4, 0x0d8, 0x0dc, 0x0e0, 0x0e4, 0x0e8, 0x0ec, 0x0f0,
            0x0f4, 0x0f8, 0x0fc, 0x100, 0x104, 0x108, 0x10c, 0x110, 0x114, 0x118, 0x11c, 0x120,
            0x124, 0x128, 0x12c, 0x130, 0x134, 0x138, 0x13c, 0x140, 0x144, 0x148, 0x14c, 0x150,
            0x154, 0x158, 0x1ec, 0x1f0, 0x1f4, 0x1f8, 0x1fc, 0x200, 0x204, 0x1d4, 0x1d8, 0x1dc,
            0x1e0, 0x1e4, 0x174, 0x170, 0x16c, 0x168, 0x17c, 0x178, 0x1b8, 0x1bc, 0x1c0, 0x1c4,
            0x1c8, 0x1cc, 0x1d0, 0x20c, 0x210, 0x214, 0x218, 0x21c, 0x220, 0x224, 0x228, 0x22c,
            0x230, 0x234, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c,
        ];
        // SAFETY: All fields accept zero and this is exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        let base = &registers as *const _ as usize;
        for (pin, offset) in offsets.into_iter().enumerate() {
            let register: &RW<u32> = &registers.gpio[pin];
            assert_eq!(register as *const _ as usize - base, offset, "GPIO{pin}");
        }
    }

    #[test]
    #[should_panic(expected = "GPIO index out of bounds")]
    fn gpio_index_rejects_128() {
        // SAFETY: All fields accept zero and this is exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        let _ = &registers.gpio[128];
    }

    #[test]
    #[should_panic(expected = "GPIO index out of bounds")]
    fn gpio_index_rejects_usize_max() {
        // SAFETY: All fields accept zero and this is exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        let _ = &registers.gpio[usize::MAX];
    }
}
