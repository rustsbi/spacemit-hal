//! K1/M1 APBC register layout.

use super::{TwsiClockReset, UartClockReset};
use volatile_register::{RW, WO};

// Offsets: https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// The vendor's APBC_UART1_CLK_RST names Linux UART0, not secure-domain UART1.
// UART1 and the real-time-domain UARTs use other clock/reset controllers.
// TWSI3 is in APBC2, not this APBC block; RCPU I2C uses its own controller.
// ccu-k1.c documents TWSI8 reads returning zero; expose writes only, never RMW.
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k1.c

/// K1/M1 APBC registers.
#[repr(C)]
pub struct RegisterBlock {
    /// UART0 clock and reset control.
    pub uart0_clock_reset: RW<UartClockReset>,
    /// UART2 clock and reset control.
    pub uart2_clock_reset: RW<UartClockReset>,
    /// GPIO clock and reset.
    pub gpio_clock_reset: RW<u32>,
    /// PWM0 clock and reset.
    pub pwm0_clock_reset: RW<u32>,
    /// PWM1 clock and reset.
    pub pwm1_clock_reset: RW<u32>,
    /// PWM2 clock and reset.
    pub pwm2_clock_reset: RW<u32>,
    /// PWM3 clock and reset.
    pub pwm3_clock_reset: RW<u32>,
    _padding_0x01c: [u32; 1],
    /// TWSI8 (I²C8) clock and reset control, whose readback is unusable.
    pub twsi8_clock_reset: WO<TwsiClockReset>,
    /// UART3 clock and reset control.
    pub uart3_clock_reset: RW<UartClockReset>,
    /// RTC clock and reset.
    pub rtc_clock_reset: RW<u32>,
    /// TWSI0 (I²C0) clock and reset control.
    pub twsi0_clock_reset: RW<TwsiClockReset>,
    /// TWSI1 (I²C1) clock and reset control.
    pub twsi1_clock_reset: RW<TwsiClockReset>,
    /// TIMER1 clock and reset.
    pub timer1_clock_reset: RW<u32>,
    /// TWSI2 (I²C2) clock and reset control.
    pub twsi2_clock_reset: RW<TwsiClockReset>,
    /// AIB clock and reset.
    pub aib_clock_reset: RW<u32>,
    /// TWSI4 (I²C4) clock and reset control.
    pub twsi4_clock_reset: RW<TwsiClockReset>,
    /// TIMER2 clock and reset.
    pub timer2_clock_reset: RW<u32>,
    /// ONEWIRE clock and reset.
    pub onewire_clock_reset: RW<u32>,
    /// TWSI5 (I²C5) clock and reset control.
    pub twsi5_clock_reset: RW<TwsiClockReset>,
    _padding_0x050: [u32; 2],
    /// DRO clock and reset.
    pub dro_clock_reset: RW<u32>,
    /// IR clock and reset.
    pub ir_clock_reset: RW<u32>,
    /// TWSI6 (I²C6) clock and reset control.
    pub twsi6_clock_reset: RW<TwsiClockReset>,
    /// Generic counter clock source selection.
    pub counter_clock_control: RW<CounterClockControl>,
    /// TWSI7 (I²C7) clock and reset control.
    pub twsi7_clock_reset: RW<TwsiClockReset>,
    /// TSENSOR clock and reset.
    pub tsensor_clock_reset: RW<u32>,
    /// UART4 clock and reset control.
    pub uart4_clock_reset: RW<UartClockReset>,
    /// UART5 clock and reset control.
    pub uart5_clock_reset: RW<UartClockReset>,
    /// UART6 clock and reset control.
    pub uart6_clock_reset: RW<UartClockReset>,
    /// SSP3 clock and reset.
    pub ssp3_clock_reset: RW<u32>,
    /// I2S0 clock and reset.
    pub i2s0_clock_reset: RW<u32>,
    /// I2S1 clock and reset.
    pub i2s1_clock_reset: RW<u32>,
    _padding_0x088: [u32; 2],
    /// IPC clock and reset.
    pub ipc_clock_reset: RW<u32>,
    /// UART7 clock and reset control.
    pub uart7_clock_reset: RW<UartClockReset>,
    /// UART8 clock and reset control.
    pub uart8_clock_reset: RW<UartClockReset>,
    /// UART9 clock and reset control.
    pub uart9_clock_reset: RW<UartClockReset>,
    /// CAN0 clock and reset.
    pub can0_clock_reset: RW<u32>,
    _padding_0x0a4: [u32; 1],
    /// PWM4 clock and reset.
    pub pwm4_clock_reset: RW<u32>,
    /// PWM5 clock and reset.
    pub pwm5_clock_reset: RW<u32>,
    /// PWM6 clock and reset.
    pub pwm6_clock_reset: RW<u32>,
    /// PWM7 clock and reset.
    pub pwm7_clock_reset: RW<u32>,
    /// PWM8 clock and reset.
    pub pwm8_clock_reset: RW<u32>,
    /// PWM9 clock and reset.
    pub pwm9_clock_reset: RW<u32>,
    /// PWM10 clock and reset.
    pub pwm10_clock_reset: RW<u32>,
    /// PWM11 clock and reset.
    pub pwm11_clock_reset: RW<u32>,
    /// PWM12 clock and reset.
    pub pwm12_clock_reset: RW<u32>,
    /// PWM13 clock and reset.
    pub pwm13_clock_reset: RW<u32>,
    /// PWM14 clock and reset.
    pub pwm14_clock_reset: RW<u32>,
    /// PWM15 clock and reset.
    pub pwm15_clock_reset: RW<u32>,
    /// PWM16 clock and reset.
    pub pwm16_clock_reset: RW<u32>,
    /// PWM17 clock and reset.
    pub pwm17_clock_reset: RW<u32>,
    /// PWM18 clock and reset.
    pub pwm18_clock_reset: RW<u32>,
    /// PWM19 clock and reset.
    pub pwm19_clock_reset: RW<u32>,
    _padding_0x0e8: [u32; 966],
}

/// K1 `APBC_COUNTER_CLK_RST` (User Manual §9.2.4.3.20, p. 202).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CounterClockControl(u32);

impl CounterClockControl {
    /// Creates a value from register bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether software selects the 24 MHz reference without automatic switching.
    #[inline]
    pub const fn is_reference_selected(self) -> bool {
        // Bit 0: FREQ_HW_CTRL; bit 1: FREQ_SW_SEL (0 = 24 MHz, 1 = 32 kHz).
        self.0 & 3 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, uart0_clock_reset), 0x000);
        assert_eq!(offset_of!(RegisterBlock, uart2_clock_reset), 0x004);
        assert_eq!(offset_of!(RegisterBlock, twsi8_clock_reset), 0x020);
        assert_eq!(offset_of!(RegisterBlock, uart3_clock_reset), 0x024);
        assert_eq!(offset_of!(RegisterBlock, twsi0_clock_reset), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, twsi1_clock_reset), 0x030);
        assert_eq!(offset_of!(RegisterBlock, twsi2_clock_reset), 0x038);
        assert_eq!(offset_of!(RegisterBlock, twsi4_clock_reset), 0x040);
        assert_eq!(offset_of!(RegisterBlock, twsi5_clock_reset), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, twsi6_clock_reset), 0x060);
        assert_eq!(offset_of!(RegisterBlock, counter_clock_control), 0x064);
        assert_eq!(offset_of!(RegisterBlock, twsi7_clock_reset), 0x068);
        assert_eq!(offset_of!(RegisterBlock, uart4_clock_reset), 0x070);
        assert_eq!(offset_of!(RegisterBlock, uart5_clock_reset), 0x074);
        assert_eq!(offset_of!(RegisterBlock, uart6_clock_reset), 0x078);
        assert_eq!(offset_of!(RegisterBlock, uart7_clock_reset), 0x094);
        assert_eq!(offset_of!(RegisterBlock, uart8_clock_reset), 0x098);
        assert_eq!(offset_of!(RegisterBlock, uart9_clock_reset), 0x09c);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
