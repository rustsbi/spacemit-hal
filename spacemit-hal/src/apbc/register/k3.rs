//! K3 APBC register layout.

use super::{TwsiClockReset, UartClockReset};
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/soc/spacemit/k3-syscon.h
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/17_clock_reset.md
// UART1 and TWSI3 use APBC2; real-time peripherals use separate controllers.

/// K3 APBC registers.
#[repr(C)]
pub struct RegisterBlock {
    /// UART0 clock and reset control.
    pub uart0_clock_reset: RW<UartClockReset>,
    /// UART2 clock and reset control.
    pub uart2_clock_reset: RW<UartClockReset>,
    /// GPIO clock and reset control.
    pub gpio_clock_reset: RW<u32>,
    /// PWM0 clock and reset control.
    pub pwm0_clock_reset: RW<u32>,
    /// PWM1 clock and reset control.
    pub pwm1_clock_reset: RW<u32>,
    /// PWM2 clock and reset control.
    pub pwm2_clock_reset: RW<u32>,
    /// PWM3 clock and reset control.
    pub pwm3_clock_reset: RW<u32>,
    /// IR1 clock and reset control.
    pub ir1_clock_reset: RW<u32>,
    /// TWSI8 clock and reset control.
    pub twsi8_clock_reset: RW<TwsiClockReset>,
    /// UART3 clock and reset control.
    pub uart3_clock_reset: RW<UartClockReset>,
    /// RTC clock and reset control.
    pub rtc_clock_reset: RW<u32>,
    /// TWSI0 clock and reset control.
    pub twsi0_clock_reset: RW<TwsiClockReset>,
    /// TWSI1 clock and reset control.
    pub twsi1_clock_reset: RW<TwsiClockReset>,
    /// TIMERS0 clock and reset control.
    pub timers0_clock_reset: RW<u32>,
    /// TWSI2 clock and reset control.
    pub twsi2_clock_reset: RW<TwsiClockReset>,
    /// AIB clock and reset control.
    pub aib_clock_reset: RW<u32>,
    /// TWSI4 clock and reset control.
    pub twsi4_clock_reset: RW<TwsiClockReset>,
    /// TIMERS1 clock and reset control.
    pub timers1_clock_reset: RW<u32>,
    /// ONEWIRE clock and reset control.
    pub onewire_clock_reset: RW<u32>,
    /// TWSI5 clock and reset control.
    pub twsi5_clock_reset: RW<TwsiClockReset>,
    _padding_0x050: [u32; 2],
    /// DRO clock and reset control.
    pub dro_clock_reset: RW<u32>,
    /// IR0 clock and reset control.
    pub ir0_clock_reset: RW<u32>,
    /// TWSI6 clock and reset control.
    pub twsi6_clock_reset: RW<TwsiClockReset>,
    /// Generic counter clock selection.
    pub counter_clk_sel: RW<u32>,
    _padding_0x068: [u32; 1],
    /// TSEN clock and reset control.
    pub tsen_clock_reset: RW<u32>,
    /// UART4 clock and reset control.
    pub uart4_clock_reset: RW<UartClockReset>,
    /// UART5 clock and reset control.
    pub uart5_clock_reset: RW<UartClockReset>,
    /// UART6 clock and reset control.
    pub uart6_clock_reset: RW<UartClockReset>,
    /// SSP3 clock and reset control.
    pub ssp3_clock_reset: RW<u32>,
    /// SSPA0 clock and reset control.
    pub sspa0_clock_reset: RW<u32>,
    /// SSPA1 clock and reset control.
    pub sspa1_clock_reset: RW<u32>,
    /// SSPA2 clock and reset control.
    pub sspa2_clock_reset: RW<u32>,
    /// SSPA3 clock and reset control.
    pub sspa3_clock_reset: RW<u32>,
    /// IPC_AP2AUD clock and reset control.
    pub ipc_ap2aud_clock_reset: RW<u32>,
    /// UART7 clock and reset control.
    pub uart7_clock_reset: RW<UartClockReset>,
    /// UART8 clock and reset control.
    pub uart8_clock_reset: RW<UartClockReset>,
    /// UART9 clock and reset control.
    pub uart9_clock_reset: RW<UartClockReset>,
    /// CAN0 clock and reset control.
    pub can0_clock_reset: RW<u32>,
    /// CAN1 clock and reset control.
    pub can1_clock_reset: RW<u32>,
    /// PWM4 clock and reset control.
    pub pwm4_clock_reset: RW<u32>,
    /// PWM5 clock and reset control.
    pub pwm5_clock_reset: RW<u32>,
    /// PWM6 clock and reset control.
    pub pwm6_clock_reset: RW<u32>,
    /// PWM7 clock and reset control.
    pub pwm7_clock_reset: RW<u32>,
    /// PWM8 clock and reset control.
    pub pwm8_clock_reset: RW<u32>,
    /// PWM9 clock and reset control.
    pub pwm9_clock_reset: RW<u32>,
    /// PWM10 clock and reset control.
    pub pwm10_clock_reset: RW<u32>,
    /// PWM11 clock and reset control.
    pub pwm11_clock_reset: RW<u32>,
    /// PWM12 clock and reset control.
    pub pwm12_clock_reset: RW<u32>,
    /// PWM13 clock and reset control.
    pub pwm13_clock_reset: RW<u32>,
    /// PWM14 clock and reset control.
    pub pwm14_clock_reset: RW<u32>,
    /// PWM15 clock and reset control.
    pub pwm15_clock_reset: RW<u32>,
    /// PWM16 clock and reset control.
    pub pwm16_clock_reset: RW<u32>,
    /// PWM17 clock and reset control.
    pub pwm17_clock_reset: RW<u32>,
    /// PWM18 clock and reset control.
    pub pwm18_clock_reset: RW<u32>,
    /// PWM19 clock and reset control.
    pub pwm19_clock_reset: RW<u32>,
    _padding_0x0e8: [u32; 13],
    /// TIMERS2 clock and reset control.
    pub timers2_clock_reset: RW<u32>,
    /// TIMERS3 clock and reset control.
    pub timers3_clock_reset: RW<u32>,
    /// TIMERS4 clock and reset control.
    pub timers4_clock_reset: RW<u32>,
    /// TIMERS5 clock and reset control.
    pub timers5_clock_reset: RW<u32>,
    /// TIMERS6 clock and reset control.
    pub timers6_clock_reset: RW<u32>,
    /// TIMERS7 clock and reset control.
    pub timers7_clock_reset: RW<u32>,
    _padding_0x134: [u32; 5],
    /// CAN2 clock and reset control.
    pub can2_clock_reset: RW<u32>,
    /// CAN3 clock and reset control.
    pub can3_clock_reset: RW<u32>,
    /// CAN4 clock and reset control.
    pub can4_clock_reset: RW<u32>,
    /// UART10 clock and reset control.
    pub uart10_clock_reset: RW<UartClockReset>,
    /// SSP0 clock and reset control.
    pub ssp0_clock_reset: RW<u32>,
    /// SSP1 clock and reset control.
    pub ssp1_clock_reset: RW<u32>,
    /// SSPA4 clock and reset control.
    pub sspa4_clock_reset: RW<u32>,
    /// SSPA5 clock and reset control.
    pub sspa5_clock_reset: RW<u32>,
    _padding_0x168: [u32; 934],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, uart0_clock_reset), 0x0);
        assert_eq!(offset_of!(RegisterBlock, uart2_clock_reset), 0x4);
        assert_eq!(offset_of!(RegisterBlock, gpio_clock_reset), 0x8);
        assert_eq!(offset_of!(RegisterBlock, pwm0_clock_reset), 0xc);
        assert_eq!(offset_of!(RegisterBlock, pwm1_clock_reset), 0x10);
        assert_eq!(offset_of!(RegisterBlock, pwm2_clock_reset), 0x14);
        assert_eq!(offset_of!(RegisterBlock, pwm3_clock_reset), 0x18);
        assert_eq!(offset_of!(RegisterBlock, ir1_clock_reset), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, twsi8_clock_reset), 0x20);
        assert_eq!(offset_of!(RegisterBlock, uart3_clock_reset), 0x24);
        assert_eq!(offset_of!(RegisterBlock, rtc_clock_reset), 0x28);
        assert_eq!(offset_of!(RegisterBlock, twsi0_clock_reset), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, twsi1_clock_reset), 0x30);
        assert_eq!(offset_of!(RegisterBlock, timers0_clock_reset), 0x34);
        assert_eq!(offset_of!(RegisterBlock, twsi2_clock_reset), 0x38);
        assert_eq!(offset_of!(RegisterBlock, aib_clock_reset), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, twsi4_clock_reset), 0x40);
        assert_eq!(offset_of!(RegisterBlock, timers1_clock_reset), 0x44);
        assert_eq!(offset_of!(RegisterBlock, onewire_clock_reset), 0x48);
        assert_eq!(offset_of!(RegisterBlock, twsi5_clock_reset), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, dro_clock_reset), 0x58);
        assert_eq!(offset_of!(RegisterBlock, ir0_clock_reset), 0x5c);
        assert_eq!(offset_of!(RegisterBlock, twsi6_clock_reset), 0x60);
        assert_eq!(offset_of!(RegisterBlock, counter_clk_sel), 0x64);
        assert_eq!(offset_of!(RegisterBlock, tsen_clock_reset), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, uart4_clock_reset), 0x70);
        assert_eq!(offset_of!(RegisterBlock, uart5_clock_reset), 0x74);
        assert_eq!(offset_of!(RegisterBlock, uart6_clock_reset), 0x78);
        assert_eq!(offset_of!(RegisterBlock, ssp3_clock_reset), 0x7c);
        assert_eq!(offset_of!(RegisterBlock, sspa0_clock_reset), 0x80);
        assert_eq!(offset_of!(RegisterBlock, sspa1_clock_reset), 0x84);
        assert_eq!(offset_of!(RegisterBlock, sspa2_clock_reset), 0x88);
        assert_eq!(offset_of!(RegisterBlock, sspa3_clock_reset), 0x8c);
        assert_eq!(offset_of!(RegisterBlock, ipc_ap2aud_clock_reset), 0x90);
        assert_eq!(offset_of!(RegisterBlock, uart7_clock_reset), 0x94);
        assert_eq!(offset_of!(RegisterBlock, uart8_clock_reset), 0x98);
        assert_eq!(offset_of!(RegisterBlock, uart9_clock_reset), 0x9c);
        assert_eq!(offset_of!(RegisterBlock, can0_clock_reset), 0xa0);
        assert_eq!(offset_of!(RegisterBlock, can1_clock_reset), 0xa4);
        assert_eq!(offset_of!(RegisterBlock, pwm4_clock_reset), 0xa8);
        assert_eq!(offset_of!(RegisterBlock, pwm5_clock_reset), 0xac);
        assert_eq!(offset_of!(RegisterBlock, pwm6_clock_reset), 0xb0);
        assert_eq!(offset_of!(RegisterBlock, pwm7_clock_reset), 0xb4);
        assert_eq!(offset_of!(RegisterBlock, pwm8_clock_reset), 0xb8);
        assert_eq!(offset_of!(RegisterBlock, pwm9_clock_reset), 0xbc);
        assert_eq!(offset_of!(RegisterBlock, pwm10_clock_reset), 0xc0);
        assert_eq!(offset_of!(RegisterBlock, pwm11_clock_reset), 0xc4);
        assert_eq!(offset_of!(RegisterBlock, pwm12_clock_reset), 0xc8);
        assert_eq!(offset_of!(RegisterBlock, pwm13_clock_reset), 0xcc);
        assert_eq!(offset_of!(RegisterBlock, pwm14_clock_reset), 0xd0);
        assert_eq!(offset_of!(RegisterBlock, pwm15_clock_reset), 0xd4);
        assert_eq!(offset_of!(RegisterBlock, pwm16_clock_reset), 0xd8);
        assert_eq!(offset_of!(RegisterBlock, pwm17_clock_reset), 0xdc);
        assert_eq!(offset_of!(RegisterBlock, pwm18_clock_reset), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, pwm19_clock_reset), 0xe4);
        assert_eq!(offset_of!(RegisterBlock, timers2_clock_reset), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, timers3_clock_reset), 0x120);
        assert_eq!(offset_of!(RegisterBlock, timers4_clock_reset), 0x124);
        assert_eq!(offset_of!(RegisterBlock, timers5_clock_reset), 0x128);
        assert_eq!(offset_of!(RegisterBlock, timers6_clock_reset), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, timers7_clock_reset), 0x130);
        assert_eq!(offset_of!(RegisterBlock, can2_clock_reset), 0x148);
        assert_eq!(offset_of!(RegisterBlock, can3_clock_reset), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, can4_clock_reset), 0x150);
        assert_eq!(offset_of!(RegisterBlock, uart10_clock_reset), 0x154);
        assert_eq!(offset_of!(RegisterBlock, ssp0_clock_reset), 0x158);
        assert_eq!(offset_of!(RegisterBlock, ssp1_clock_reset), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, sspa4_clock_reset), 0x160);
        assert_eq!(offset_of!(RegisterBlock, sspa5_clock_reset), 0x164);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
