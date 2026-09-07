//! K1/M1 generic counter registers.
use super::Control;
use volatile_register::RW;

// K1 User Manual, section 8.2.4, Generic Counter Registers Description.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k1/k1_docs/k1_usermanual/8.CPU_System.md
// Base: 0xd500_1000. No K3 layout is assumed.

/// K1/M1 generic counter control registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Counter enable control.
    pub control: RW<Control>,
    /// Counter status (`CNTSR_REG`), including the debug-halted state.
    pub status: RW<u32>,
    /// Counter bits 31:0 (`CNTCVLW_REG`). Do not modify a live system timebase.
    pub value_low: RW<u32>,
    /// Counter bits 63:32 (`CNTCVUP_REG`). Do not modify a live system timebase.
    pub value_high: RW<u32>,
    _reserved_0x10: [u32; 4],
    /// Frequency information in ticks per second (`CNTFID_REG`).
    ///
    /// This writable word is not used to infer or configure the source clock.
    pub frequency_id: RW<u32>,
}

impl RegisterBlock {
    /// Reads a coherent 64-bit counter value using ordered 32-bit MMIO reads.
    ///
    /// Reads high/low/high and retries when the high word changes, avoiding a
    /// torn sample if the low word wraps. This works on both RV32 and RV64 and
    /// does not require `time` CSR access. Concurrent software writes to the
    /// counter value must be excluded; continuous rewrites can prevent progress.
    #[inline]
    pub fn value(&self) -> u64 {
        read_value(
            || {
                let high = self.value_high.read();
                super::super::io_fence();
                high
            },
            || {
                let low = self.value_low.read();
                super::super::io_fence();
                low
            },
        )
    }
}

#[inline]
fn read_value(mut high: impl FnMut() -> u32, mut low: impl FnMut() -> u32) -> u64 {
    loop {
        let before = high();
        let lower = low();
        if before == high() {
            return (u64::from(before) << 32) | u64::from(lower);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0);
        assert_eq!(offset_of!(RegisterBlock, status), 0x04);
        assert_eq!(offset_of!(RegisterBlock, value_low), 0x08);
        assert_eq!(offset_of!(RegisterBlock, value_high), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, frequency_id), 0x20);
        assert_eq!(size_of::<RegisterBlock>(), 0x24);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn reads_both_mmio_words_without_changing_them() {
        // SAFETY: All fields accept zero in exclusively owned test memory.
        let registers: RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: These are exclusively owned test registers, not live MMIO.
        unsafe {
            registers.value_low.write(0x7654_3210);
            registers.value_high.write(0xfedc_ba98);
        }
        assert_eq!(registers.value(), 0xfedc_ba98_7654_3210);
        assert_eq!(registers.value(), 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn retries_a_torn_sample_including_full_counter_wrap() {
        for (before, after) in [(7, 8), (u32::MAX, 0)] {
            let mut highs = [before, after, after, after].into_iter();
            let mut lows = [u32::MAX, 1].into_iter();
            let value = read_value(|| highs.next().unwrap(), || lows.next().unwrap());
            assert_eq!(value, (u64::from(after) << 32) | 1);
            assert!(highs.next().is_none());
            assert!(lows.next().is_none());
        }
    }
}
