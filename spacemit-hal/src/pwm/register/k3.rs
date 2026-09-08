//! K3 pulse-width modulation registers.

use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/16_peripherals/pwm.md

/// K3 pulse-width modulation registers.
#[repr(C)]
pub struct RegisterBlock {
    /// PWM control (PWM_CR).
    pub control: RW<u32>,
    /// Duty cycle (PWM_DCR).
    pub duty_cycle: RW<u32>,
    /// Period count (PWM_PCR).
    pub period: RW<u32>,
    _padding_0x00c: [u32; 1],
    /// Output pulse count (PWM_OUTCNT).
    pub output_count: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, duty_cycle), 0x4);
        assert_eq!(offset_of!(RegisterBlock, period), 0x8);
        assert_eq!(offset_of!(RegisterBlock, output_count), 0x10);
        assert_eq!(size_of::<RegisterBlock>(), 0x14);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
