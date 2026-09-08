//! K1/M1 pulse-width modulation registers.

use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/pwm/pwm-pxa.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/arch/riscv/boot/dts/spacemit/k1.dtsi

/// K1/M1 pulse-width modulation registers.
#[repr(C)]
pub struct RegisterBlock {
    /// PWM control (PWM_CR).
    pub control: RW<u32>,
    /// Duty cycle (PWM_DCR).
    pub duty_cycle: RW<u32>,
    /// Period count (PWM_PCR).
    pub period: RW<u32>,
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
        assert_eq!(size_of::<RegisterBlock>(), 0xc);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
