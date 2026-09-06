//! K3 multi-function pad register layout.

use volatile_register::RW;

// Pin offsets: spacemit_k3_pin_to_offset in Linux pinctrl-k1.c.
// https://github.com/torvalds/linux/blob/master/drivers/pinctrl/spacemit/pinctrl-k1.c
// GPIO0..127 occupy offsets 0x000..0x1fc; subsequent pads have dedicated names.
// Bit 6 (EDGE_CLEAR) is RW: preserve its edge-disable state when changing mux.
// K3 pad offsets and electrical field encodings differ from K1.

/// K3 multi-function pad registers.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO0 through GPIO127 pad configuration, indexed by GPIO number.
    pub gpio: [RW<u32>; 128],
    _reserved_0x200: [u32; 896],
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

        // SAFETY: All fields accept zero and this is exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        let gpio: &[RW<u32>; 128] = &registers.gpio;
        let base = &registers as *const _ as usize;
        for (pin, register) in gpio.iter().enumerate() {
            assert_eq!(register as *const _ as usize - base, pin * 4);
        }
    }
}
