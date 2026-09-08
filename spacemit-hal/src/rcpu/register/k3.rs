//! K3 real-time CPU system registers.

use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/soc/spacemit/k3-syscon.h

/// K3 real-time CPU system registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 15],
    /// uart 14m clock.
    pub uart_14m_clock: RW<u32>,
    _padding_0x040: [u32; 1],
    /// i2s1 system clock.
    pub i2s1_system_clock: RW<u32>,
    /// ir0 clock reset.
    pub ir0_clock_reset: RW<u32>,
    /// can0 clock reset.
    pub can0_clock_reset: RW<u32>,
    _padding_0x050: [u32; 8],
    /// i2s0 system clock.
    pub i2s0_system_clock: RW<u32>,
    _padding_0x074: [u32; 25],
    /// uart 58m clock.
    pub uart_58m_clock: RW<u32>,
    /// espi clock reset.
    pub espi_clock_reset: RW<u32>,
    _padding_0x0e0: [u32; 1],
    /// gmac clock reset.
    pub gmac_clock_reset: RW<u32>,
    _padding_0x0e8: [u32; 1],
    /// ir1 clock reset.
    pub ir1_clock_reset: RW<u32>,
    /// can1 clock reset.
    pub can1_clock_reset: RW<u32>,
    /// can2 clock reset.
    pub can2_clock_reset: RW<u32>,
    /// can3 clock reset.
    pub can3_clock_reset: RW<u32>,
    /// can4 clock reset.
    pub can4_clock_reset: RW<u32>,
}

/// Uart Clock Registers.
#[repr(C)]
pub struct UartClockRegisters {
    /// Per-instance clock and reset controls.
    pub clock_reset: [RW<u32>; 6],
}

/// Spi Clock Registers.
#[repr(C)]
pub struct SpiClockRegisters {
    /// Per-instance clock and reset controls.
    pub clock_reset: [RW<u32>; 3],
}

/// I2c Clock Registers.
#[repr(C)]
pub struct I2cClockRegisters {
    /// Per-instance clock and reset controls.
    pub clock_reset: [RW<u32>; 3],
}

/// Pwm Clock Registers.
#[repr(C)]
pub struct PwmClockRegisters {
    /// Per-instance clock and reset controls.
    pub clock_reset: [RW<u32>; 10],
}

/// Real-time audio clock registers.
#[repr(C)]
pub struct AudioClockRegisters {
    _padding_0x000: [u32; 17],
    /// i2s2 system clock.
    pub i2s2_system_clock: RW<u32>,
    _padding_0x048: [u32; 3],
    /// i2s3 system clock.
    pub i2s3_system_clock: RW<u32>,
    _padding_0x058: [u32; 2],
    /// i2s0 clock reset.
    pub i2s0_clock_reset: RW<u32>,
    /// i2s1 clock reset.
    pub i2s1_clock_reset: RW<u32>,
    /// i2s2 clock reset.
    pub i2s2_clock_reset: RW<u32>,
    /// i2s3 clock reset.
    pub i2s3_clock_reset: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(UartClockRegisters, clock_reset), 0x000);
        assert_eq!(size_of::<UartClockRegisters>(), 0x018);
        assert_eq!(offset_of!(SpiClockRegisters, clock_reset), 0x000);
        assert_eq!(size_of::<SpiClockRegisters>(), 0x00c);
        assert_eq!(offset_of!(I2cClockRegisters, clock_reset), 0x000);
        assert_eq!(size_of::<I2cClockRegisters>(), 0x00c);
        assert_eq!(offset_of!(PwmClockRegisters, clock_reset), 0x000);
        assert_eq!(size_of::<PwmClockRegisters>(), 0x028);
        assert_eq!(offset_of!(AudioClockRegisters, i2s2_system_clock), 0x044);
        assert_eq!(offset_of!(AudioClockRegisters, i2s3_system_clock), 0x054);
        assert_eq!(offset_of!(AudioClockRegisters, i2s0_clock_reset), 0x060);
        assert_eq!(offset_of!(AudioClockRegisters, i2s1_clock_reset), 0x064);
        assert_eq!(offset_of!(AudioClockRegisters, i2s2_clock_reset), 0x068);
        assert_eq!(offset_of!(AudioClockRegisters, i2s3_clock_reset), 0x06c);
        assert_eq!(size_of::<AudioClockRegisters>(), 0x070);
        assert_eq!(offset_of!(RegisterBlock, uart_14m_clock), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, i2s1_system_clock), 0x044);
        assert_eq!(offset_of!(RegisterBlock, ir0_clock_reset), 0x048);
        assert_eq!(offset_of!(RegisterBlock, can0_clock_reset), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, i2s0_system_clock), 0x070);
        assert_eq!(offset_of!(RegisterBlock, uart_58m_clock), 0x0d8);
        assert_eq!(offset_of!(RegisterBlock, espi_clock_reset), 0x0dc);
        assert_eq!(offset_of!(RegisterBlock, gmac_clock_reset), 0x0e4);
        assert_eq!(offset_of!(RegisterBlock, ir1_clock_reset), 0x0ec);
        assert_eq!(offset_of!(RegisterBlock, can1_clock_reset), 0x0f0);
        assert_eq!(offset_of!(RegisterBlock, can2_clock_reset), 0x0f4);
        assert_eq!(offset_of!(RegisterBlock, can3_clock_reset), 0x0f8);
        assert_eq!(offset_of!(RegisterBlock, can4_clock_reset), 0x0fc);
        assert_eq!(size_of::<RegisterBlock>(), 0x100);
    }
}
