//! K1/M1 peripheral ownership and addresses.

use spacemit_hal::{
    adma, ahbdma, apbc, apbc2, apbs, apmu, can, ccic, ciu, counter, dciu, dpu, dsi, emac, gpio,
    i2c, ir, mailbox, mfpr, mpmu, onewire, pcie, pdma, plic, pwm, qspi, rcpu, ri2s, rpmu, rtc, sdh,
    spi, timer, trng, tsensor, uart, usb2, usb2_phy, usb3, v2d, vpu,
};

// Address map: Linux k1.dtsi and the vendor k1-x.dtsi (UART1 and R_UART0/1).
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// https://gitee.com/spacemit-buildroot/linux-6.6/blob/k1-bl-v2.2.y/arch/riscv/boot/dts/spacemit/k1-x.dtsi
// UART1 is in the secure domain; R_UART1 is at 0xc088_d000, unlike K3.
// Additional instances: K1 user manual, section 6.2.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k1/k1_docs/k1_usermanual/6.Address_Mapping.md
// SDH0..2 are zero-based software IDs (SDH1..3 in the manual).
// Camera and display instances use the vendor k1-x-camera-sdk, k1-x-lcd and k1-x-hdmi DTSIs.
// https://github.com/spacemit-com/linux-6.6/tree/k1-bl-v2.2.y/arch/riscv/boot/dts/spacemit
// CSI PHY/CCIC and USB OTG/UDC/EHCI aliases each have one token.

soc! {
    /// I2C0 (TWSI0) peripheral.
    pub struct I2C0 => 0xd401_0800, i2c::RegisterBlock;
    /// I2C1 (TWSI1) peripheral.
    pub struct I2C1 => 0xd401_1000, i2c::RegisterBlock;
    /// I2C2 (TWSI2) peripheral.
    pub struct I2C2 => 0xd401_2000, i2c::RegisterBlock;
    /// I2C4 (TWSI4) peripheral.
    pub struct I2C4 => 0xd401_2800, i2c::RegisterBlock;
    /// I2C5 (TWSI5) peripheral.
    pub struct I2C5 => 0xd401_3800, i2c::RegisterBlock;
    /// I2C6 (TWSI6) peripheral.
    pub struct I2C6 => 0xd401_8800, i2c::RegisterBlock;
    /// I2C7 (TWSI7) peripheral.
    pub struct I2C7 => 0xd401_d000, i2c::RegisterBlock;
    /// I2C8 (TWSI8) peripheral.
    pub struct I2C8 => 0xd401_d800, i2c::RegisterBlock;
    /// PLL clock-control peripheral.
    pub struct APBS => 0xd409_0000, apbs::k1::RegisterBlock;
    /// Main power-management peripheral.
    pub struct MPMU => 0xd405_0000, mpmu::k1::RegisterBlock;
    /// Multi-function pad peripheral.
    pub struct MFPR => 0xd401_e000, mfpr::k1::RegisterBlock;
    /// Quad-SPI memory-controller peripheral.
    pub struct QSPI => 0xd420_c000, qspi::RegisterBlock;
    /// Generic counter peripheral.
    pub struct COUNTER => 0xd500_1000, counter::k1::RegisterBlock;
    /// APB clock and reset peripheral.
    pub struct APBC => 0xd401_5000, apbc::k1::RegisterBlock;
    /// Application-processor power, clock, and reset peripheral.
    pub struct APMU => 0xd428_2800, apmu::k1::RegisterBlock;
    /// CPU configuration peripheral.
    pub struct CIU => 0xd428_2c00, ciu::k1::RegisterBlock;
    /// GPIO peripheral.
    pub struct GPIO => 0xd401_9000, gpio::k1::RegisterBlock;
    /// UART0 peripheral.
    pub struct UART0 => 0xd401_7000, uart::RegisterBlock;
    /// UART1 peripheral.
    pub struct UART1 => 0xf061_2000, uart::RegisterBlock;
    /// UART2 peripheral.
    pub struct UART2 => 0xd401_7100, uart::RegisterBlock;
    /// UART3 peripheral.
    pub struct UART3 => 0xd401_7200, uart::RegisterBlock;
    /// UART4 peripheral.
    pub struct UART4 => 0xd401_7300, uart::RegisterBlock;
    /// UART5 peripheral.
    pub struct UART5 => 0xd401_7400, uart::RegisterBlock;
    /// UART6 peripheral.
    pub struct UART6 => 0xd401_7500, uart::RegisterBlock;
    /// UART7 peripheral.
    pub struct UART7 => 0xd401_7600, uart::RegisterBlock;
    /// UART8 peripheral.
    pub struct UART8 => 0xd401_7700, uart::RegisterBlock;
    /// UART9 peripheral.
    pub struct UART9 => 0xd401_7800, uart::RegisterBlock;
    /// Real-time-domain UART0 peripheral.
    pub struct R_UART0 => 0xc088_1000, uart::RegisterBlock;
    /// Real-time-domain UART1 peripheral.
    pub struct R_UART1 => 0xc088_d000, uart::RegisterBlock;

    /// Secure I2C3 (TWSI3) peripheral.
    pub struct I2C3 => 0xf061_4000, i2c::RegisterBlock;
    /// PWM0 peripheral.
    pub struct PWM0 => 0xd401_a000, pwm::k1::RegisterBlock;
    /// PWM1 peripheral.
    pub struct PWM1 => 0xd401_a400, pwm::k1::RegisterBlock;
    /// PWM2 peripheral.
    pub struct PWM2 => 0xd401_a800, pwm::k1::RegisterBlock;
    /// PWM3 peripheral.
    pub struct PWM3 => 0xd401_ac00, pwm::k1::RegisterBlock;
    /// PWM4 peripheral.
    pub struct PWM4 => 0xd401_b000, pwm::k1::RegisterBlock;
    /// PWM5 peripheral.
    pub struct PWM5 => 0xd401_b400, pwm::k1::RegisterBlock;
    /// PWM6 peripheral.
    pub struct PWM6 => 0xd401_b800, pwm::k1::RegisterBlock;
    /// PWM7 peripheral.
    pub struct PWM7 => 0xd401_bc00, pwm::k1::RegisterBlock;
    /// PWM8 peripheral.
    pub struct PWM8 => 0xd402_0000, pwm::k1::RegisterBlock;
    /// PWM9 peripheral.
    pub struct PWM9 => 0xd402_0400, pwm::k1::RegisterBlock;
    /// PWM10 peripheral.
    pub struct PWM10 => 0xd402_0800, pwm::k1::RegisterBlock;
    /// PWM11 peripheral.
    pub struct PWM11 => 0xd402_0c00, pwm::k1::RegisterBlock;
    /// PWM12 peripheral.
    pub struct PWM12 => 0xd402_1000, pwm::k1::RegisterBlock;
    /// PWM13 peripheral.
    pub struct PWM13 => 0xd402_1400, pwm::k1::RegisterBlock;
    /// PWM14 peripheral.
    pub struct PWM14 => 0xd402_1800, pwm::k1::RegisterBlock;
    /// PWM15 peripheral.
    pub struct PWM15 => 0xd402_1c00, pwm::k1::RegisterBlock;
    /// PWM16 peripheral.
    pub struct PWM16 => 0xd402_2000, pwm::k1::RegisterBlock;
    /// PWM17 peripheral.
    pub struct PWM17 => 0xd402_2400, pwm::k1::RegisterBlock;
    /// PWM18 peripheral.
    pub struct PWM18 => 0xd402_2800, pwm::k1::RegisterBlock;
    /// PWM19 peripheral.
    pub struct PWM19 => 0xd402_2c00, pwm::k1::RegisterBlock;
    /// Secure SSP2 / SPI peripheral.
    pub struct SPI2 => 0xf061_3000, spi::k1::RegisterBlock;
    /// SSP3 / SPI peripheral.
    pub struct SPI3 => 0xd401_c000, spi::k1::RegisterBlock;
    /// Full-duplex I2S0 peripheral.
    pub struct I2S0 => 0xd402_6000, spi::k1::RegisterBlock;
    /// Full-duplex I2S1 peripheral.
    pub struct I2S1 => 0xd402_6800, spi::k1::RegisterBlock;
    /// SD / SDIO / eMMC host 0.
    pub struct SDH0 => 0xd428_0000, sdh::k1::RegisterBlock;
    /// SD / SDIO / eMMC host 1.
    pub struct SDH1 => 0xd428_0800, sdh::k1::RegisterBlock;
    /// SD / SDIO / eMMC host 2.
    pub struct SDH2 => 0xd428_1000, sdh::k1::RegisterBlock;
    /// Timer 1 and watchdog peripheral.
    pub struct TIMER1 => 0xd401_4000, timer::k1::RegisterBlock;
    /// Timer 2 and watchdog peripheral.
    pub struct TIMER2 => 0xd401_6000, timer::k1::RegisterBlock;
    /// Non-secure peripheral DMA controller.
    pub struct PDMA => 0xd400_0000, pdma::k1::RegisterBlock;
    /// Secure APB clock and reset controller.
    pub struct APBC2 => 0xf061_0000, apbc2::k1::RegisterBlock;
    /// Real-time clock.
    pub struct RTC => 0xd401_0000, rtc::k1::RegisterBlock;
    /// Secure real-time clock.
    pub struct SEC_RTC => 0xf061_5000, rtc::k1::RegisterBlock;
    /// Real-time CPU system controller.
    pub struct RCPU => 0xc088_0000, rcpu::k1::RegisterBlock;
    /// Real-time audio clock controller.
    pub struct R_AUDIO_CLOCK => 0xc088_2000, rcpu::k1::AudioClockRegisters;
    /// Real-time power-management controller.
    pub struct RPMU => 0xc088_c000, rpmu::k1::RegisterBlock;
    /// Real-time PWM clock controller.
    pub struct R_PWM_CLOCK => 0xc088_8000, rcpu::k1::PwmClockRegisters;
    /// One-Wire bus master.
    pub struct ONEWIRE => 0xd401_1800, onewire::k1::RegisterBlock;
    /// Application-side mailbox.
    pub struct MAILBOX => 0xd401_3400, mailbox::k1::RegisterBlock;
    /// Real-time-side mailbox.
    pub struct R_MAILBOX => 0xc088_a000, mailbox::k1::RegisterBlock;
    /// Infrared receiver.
    pub struct IR => 0xd401_7f00, ir::k1::RegisterBlock;
    /// Real-time infrared receiver.
    pub struct R_IR => 0xc088_e000, ir::k1::RegisterBlock;
    /// Temperature sensor.
    pub struct TSENSOR => 0xd401_8000, tsensor::k1::RegisterBlock;
    /// CAN-FD controller.
    pub struct CAN0 => 0xd402_8000, can::k1::RegisterBlock;
    /// Real-time CAN-FD controller.
    pub struct R_CAN0 => 0xc087_0000, can::k1::RegisterBlock;
    /// Random generator and DMA.
    pub struct TRNG => 0xf070_3800, trng::k1::RegisterBlock;
    /// Real-time AHB DMA.
    pub struct AHBDMA => 0xc088_4000, ahbdma::k1::RegisterBlock;
    /// Ethernet MAC 0.
    pub struct EMAC0 => 0xcac8_0000, emac::k1::RegisterBlock;
    /// Ethernet MAC 1.
    pub struct EMAC1 => 0xcac8_1000, emac::k1::RegisterBlock;
    /// PMU timer and watchdog.
    pub struct PMU_TIMER => 0xd408_0000, timer::k1::RegisterBlock;
    /// Secure timer and watchdog.
    pub struct SEC_TIMER => 0xf061_6000, timer::k1::RegisterBlock;
    /// USB2 OTG controller.
    pub struct USB2_OTG => 0xc090_0100, usb2::k1::RegisterBlock;
    /// USB2 host controller.
    pub struct USB2_HOST => 0xc098_0100, usb2::k1::RegisterBlock;
    /// USB2 OTG PHY.
    pub struct USB2_PHY_OTG => 0xc094_0000, usb2_phy::k1::RegisterBlock;
    /// USB2 host PHY.
    pub struct USB2_PHY_HOST => 0xc09c_0000, usb2_phy::k1::RegisterBlock;
    /// USB3 UTMI PHY.
    pub struct USB3_UTMI => 0xc0a3_0000, usb2_phy::k1::RegisterBlock;
    /// DWC3 USB3 controller.
    pub struct USB3 => 0xc0a0_0000, usb3::RegisterBlock;
    /// Real-time PWM 0.
    pub struct R_PWM0 => 0xc088_8100, pwm::k1::RegisterBlock;
    /// Real-time PWM 1.
    pub struct R_PWM1 => 0xc088_8200, pwm::k1::RegisterBlock;
    /// Real-time PWM 2.
    pub struct R_PWM2 => 0xc088_8300, pwm::k1::RegisterBlock;
    /// Real-time PWM 3.
    pub struct R_PWM3 => 0xc088_8400, pwm::k1::RegisterBlock;
    /// Real-time PWM 4.
    pub struct R_PWM4 => 0xc088_8500, pwm::k1::RegisterBlock;
    /// Real-time PWM 5.
    pub struct R_PWM5 => 0xc088_8600, pwm::k1::RegisterBlock;
    /// Real-time PWM 6.
    pub struct R_PWM6 => 0xc088_8700, pwm::k1::RegisterBlock;
    /// Real-time PWM 7.
    pub struct R_PWM7 => 0xc088_8800, pwm::k1::RegisterBlock;
    /// Real-time PWM 8.
    pub struct R_PWM8 => 0xc088_8900, pwm::k1::RegisterBlock;
    /// Real-time PWM 9.
    pub struct R_PWM9 => 0xc088_8a00, pwm::k1::RegisterBlock;
    /// Audio DMA 0.
    pub struct ADMA0 => 0xc088_3000, adma::k1::RegisterBlock;
    /// Half-duplex audio interface 0.
    pub struct RI2S0 => 0xc088_3100, ri2s::k1::RegisterBlock;
    /// Audio DMA 1.
    pub struct ADMA1 => 0xc088_3400, adma::k1::RegisterBlock;
    /// Half-duplex audio interface 1.
    pub struct RI2S1 => 0xc088_3500, ri2s::k1::RegisterBlock;
    /// Audio DMA 2.
    pub struct ADMA2 => 0xc088_3800, adma::k1::RegisterBlock;
    /// Half-duplex audio interface 2.
    pub struct RI2S2 => 0xc088_3900, ri2s::k1::RegisterBlock;
    /// Audio DMA 3.
    pub struct ADMA3 => 0xc088_3c00, adma::k1::RegisterBlock;
    /// Half-duplex audio interface 3.
    pub struct RI2S3 => 0xc088_3d00, ri2s::k1::RegisterBlock;
    /// PCIe 0 link-management registers.
    pub struct PCIE0_LINK => 0xc0b2_0000, pcie::k1::RegisterBlock;
    /// PCIe 1 link-management registers.
    pub struct PCIE1_LINK => 0xc0c2_0000, pcie::k1::RegisterBlock;
    /// PCIe 2 link-management registers.
    pub struct PCIE2_LINK => 0xc0d2_0000, pcie::k1::RegisterBlock;
    /// DCIU peripheral.
    pub struct DCIU => 0xd844_0000, dciu::k1::RegisterBlock;
    /// DSI peripheral.
    pub struct DSI => 0xd421_a800, dsi::k1::RegisterBlock;
    /// 2D graphics engine.
    pub struct V2D => 0xc010_0000, v2d::RegisterBlock;
    /// Camera capture and CSI PHY 0.
    pub struct CCIC0 => 0xd420_a000, ccic::k1::RegisterBlock;
    /// Camera capture and CSI PHY 1.
    pub struct CCIC1 => 0xd420_a800, ccic::k1::RegisterBlock;
    /// Camera capture and CSI PHY 2.
    pub struct CCIC2 => 0xd420_6000, ccic::k1::RegisterBlock;
    /// Video processing unit.
    pub struct VPU => 0xc050_0000, vpu::RegisterBlock;
    /// Display processing unit 0.
    pub struct DPU0 => 0xc034_0000, dpu::k1::RegisterBlock;
    /// Display processing unit 1.
    pub struct DPU1 => 0xc044_0000, dpu::k1::RegisterBlock;
    /// Real-time I2C controller.
    pub struct R_I2C0 => 0xc088_7000, i2c::RegisterBlock;
    /// Real-time SPI controller 0.
    pub struct R_SPI0 => 0xc088_5000, spi::k1::RegisterBlock;
    /// Platform interrupt controller.
    pub struct PLIC => 0xe000_0000, plic::k1::RegisterBlock;
}

impl_clock_controller!(apbs, APBS, apbs::k1::RegisterBlock);
impl_clock_controller!(mpmu, MPMU, mpmu::k1::RegisterBlock);
impl_qspi!(QSPI);

impl_uart!(
    UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7, UART8, UART9, R_UART0, R_UART1
);

// SAFETY: Peripherals::take/steal grants permanent exclusive K1/M1 counter
// access, including permission to enable it without conflicting writers.
unsafe impl<'a> counter::Instance<'a> for COUNTER {
    #[inline]
    fn register_block(self) -> &'a counter::k1::RegisterBlock {
        // SAFETY: Consuming the token transfers its permanently valid mapping.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: The mutable borrow retains the token's guarantees for its lifetime.
unsafe impl<'a> counter::Instance<'a> for &'a mut COUNTER {
    #[inline]
    fn register_block(self) -> &'a counter::k1::RegisterBlock {
        self
    }
}

gpio_pads!(__new_k1);

// pinctrl_i2c2_0: function 4, pull-up, 1.8 V drive strength 0, edge detection disabled.
impl_i2c_pads!(I2C2, 84, 85, 0xc044);

/// Exclusive dedicated PWR_SCL/PWR_SDA pads, retaining their boot configuration.
pub struct PmicPads {
    _private: core::marker::PhantomData<*mut ()>,
}

// SAFETY: Peripherals::steal transfers the configured, dedicated I²C8 pads once.
unsafe impl<'a> i2c::IntoPads<'a, I2C8> for PmicPads {
    #[inline]
    fn into_i2c_pads(self) -> i2c::Pads<'a> {
        // SAFETY: Consuming this token transfers permanent dedicated-pad ownership.
        unsafe { i2c::Pads::__dedicated() }
    }
}

// SAFETY: The borrow excludes the owner and retains the same dedicated route.
unsafe impl<'a> i2c::IntoPads<'a, I2C8> for &'a mut PmicPads {
    #[inline]
    fn into_i2c_pads(self) -> i2c::Pads<'a> {
        // SAFETY: Dedicated-pad access is transferred only for this borrow.
        unsafe { i2c::Pads::__dedicated() }
    }
}

#[cfg(test)]
mod i2c_pad_tests {
    use super::*;

    #[test]
    fn owned_and_borrowed_routes() {
        fn route<'a, I: spacemit_hal::clock::I2cId, P: i2c::IntoPads<'a, I>>() {}
        route::<I2C2, (Pad<84>, Pad<85>)>();
        route::<I2C2, (&mut Pad<84>, &mut Pad<85>)>();
        route::<I2C8, PmicPads>();
        route::<I2C8, &mut PmicPads>();
        assert_eq!(core::mem::size_of::<PmicPads>(), 0);
    }
}

// UART GPIO routes: Linux k1-pinctrl.dtsi; TX and RX may use different functions.
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1-pinctrl.dtsi
impl_uart_pads! {
    (68, 2): IntoTransmit, into_uart_transmit, UART0;
    (69, 2): IntoReceive, into_uart_receive, UART0;
    (80, 3): IntoReceive, into_uart_receive, UART0;
    (104, 3): IntoTransmit, into_uart_transmit, UART0;
    (105, 3): IntoReceive, into_uart_receive, UART0;
    (108, 1): IntoTransmit, into_uart_transmit, UART0;
    (21, 1): IntoTransmit, into_uart_transmit, UART2;
    (22, 1): IntoReceive, into_uart_receive, UART2;
    (18, 2): IntoTransmit, into_uart_transmit, UART3;
    (19, 2): IntoReceive, into_uart_receive, UART3;
    (53, 4): IntoTransmit, into_uart_transmit, UART3;
    (54, 4): IntoReceive, into_uart_receive, UART3;
    (81, 2): IntoTransmit, into_uart_transmit, UART3;
    (82, 2): IntoReceive, into_uart_receive, UART3;
    (23, 2): IntoTransmit, into_uart_transmit, UART4;
    (24, 2): IntoReceive, into_uart_receive, UART4;
    (33, 2): IntoTransmit, into_uart_transmit, UART4;
    (34, 2): IntoReceive, into_uart_receive, UART4;
    (83, 3): IntoTransmit, into_uart_transmit, UART4;
    (84, 3): IntoReceive, into_uart_receive, UART4;
    (100, 4): IntoTransmit, into_uart_transmit, UART4;
    (101, 4): IntoReceive, into_uart_receive, UART4;
    (111, 4): IntoTransmit, into_uart_transmit, UART4;
    (112, 4): IntoReceive, into_uart_receive, UART4;
    (25, 2): IntoTransmit, into_uart_transmit, UART5;
    (26, 2): IntoReceive, into_uart_receive, UART5;
    (42, 2): IntoTransmit, into_uart_transmit, UART5;
    (43, 2): IntoReceive, into_uart_receive, UART5;
    (70, 4): IntoTransmit, into_uart_transmit, UART5;
    (71, 4): IntoReceive, into_uart_receive, UART5;
    (102, 3): IntoTransmit, into_uart_transmit, UART5;
    (103, 3): IntoReceive, into_uart_receive, UART5;
    (0, 2): IntoTransmit, into_uart_transmit, UART6;
    (1, 2): IntoReceive, into_uart_receive, UART6;
    (56, 2): IntoTransmit, into_uart_transmit, UART6;
    (57, 2): IntoReceive, into_uart_receive, UART6;
    (86, 2): IntoTransmit, into_uart_transmit, UART6;
    (87, 2): IntoReceive, into_uart_receive, UART6;
    (4, 2): IntoTransmit, into_uart_transmit, UART7;
    (5, 2): IntoReceive, into_uart_receive, UART7;
    (88, 2): IntoTransmit, into_uart_transmit, UART7;
    (89, 2): IntoReceive, into_uart_receive, UART7;
    (8, 2): IntoTransmit, into_uart_transmit, UART8;
    (9, 2): IntoReceive, into_uart_receive, UART8;
    (75, 4): IntoTransmit, into_uart_transmit, UART8;
    (76, 4): IntoReceive, into_uart_receive, UART8;
    (82, 4): IntoTransmit, into_uart_transmit, UART8;
    (83, 4): IntoReceive, into_uart_receive, UART8;
    (12, 2): IntoTransmit, into_uart_transmit, UART9;
    (13, 2): IntoReceive, into_uart_receive, UART9;
    (72, 2): IntoTransmit, into_uart_transmit, UART9;
    (73, 2): IntoReceive, into_uart_receive, UART9;
    (116, 3): IntoTransmit, into_uart_transmit, UART9;
    (117, 3): IntoReceive, into_uart_receive, UART9;
}

apbc_clocks! {
    APBC, apbc::k1::RegisterBlock;
    uart {
        UART0 => uart0, uart0_clock_reset;
        UART2 => uart2, uart2_clock_reset;
        UART3 => uart3, uart3_clock_reset;
        UART4 => uart4, uart4_clock_reset;
        UART5 => uart5, uart5_clock_reset;
        UART6 => uart6, uart6_clock_reset;
        UART7 => uart7, uart7_clock_reset;
        UART8 => uart8, uart8_clock_reset;
        UART9 => uart9, uart9_clock_reset;
    }
    i2c {
        I2C0 => i2c0, twsi0_clock_reset;
        I2C1 => i2c1, twsi1_clock_reset;
        I2C2 => i2c2, twsi2_clock_reset;
        I2C4 => i2c4, twsi4_clock_reset;
        I2C5 => i2c5, twsi5_clock_reset;
        I2C6 => i2c6, twsi6_clock_reset;
        I2C7 => i2c7, twsi7_clock_reset;
        I2C8 => i2c8, twsi8_clock_reset;
    }
    counter { COUNTER => counter, counter_clock_control; }
}

/// Exclusive application-hart tokens.
pub struct Harts {
    /// Hardware hart 0.
    pub hart0: crate::hart::Hart<0>,
    /// Hardware hart 1.
    pub hart1: crate::hart::Hart<1>,
    /// Hardware hart 2.
    pub hart2: crate::hart::Hart<2>,
    /// Hardware hart 3.
    pub hart3: crate::hart::Hart<3>,
    /// Hardware hart 4.
    pub hart4: crate::hart::Hart<4>,
    /// Hardware hart 5.
    pub hart5: crate::hart::Hart<5>,
    /// Hardware hart 6.
    pub hart6: crate::hart::Hart<6>,
    /// Hardware hart 7.
    pub hart7: crate::hart::Hart<7>,
}

/// K1/M1 peripheral ownership.
pub struct Peripherals {
    /// Exclusive application-hart tokens.
    pub harts: Harts,
    /// Dedicated PMIC I²C pads.
    pub pmic_pads: PmicPads,
    /// I2C0 (TWSI0) peripheral.
    pub i2c0: I2C0,
    /// I2C1 (TWSI1) peripheral.
    pub i2c1: I2C1,
    /// I2C2 (TWSI2) peripheral.
    pub i2c2: I2C2,
    /// I2C4 (TWSI4) peripheral.
    pub i2c4: I2C4,
    /// I2C5 (TWSI5) peripheral.
    pub i2c5: I2C5,
    /// I2C6 (TWSI6) peripheral.
    pub i2c6: I2C6,
    /// I2C7 (TWSI7) peripheral.
    pub i2c7: I2C7,
    /// I2C8 (TWSI8) peripheral.
    pub i2c8: I2C8,
    /// PLL clock-control peripheral.
    pub apbs: APBS,
    /// Main power-management peripheral.
    pub mpmu: MPMU,
    /// Quad-SPI memory-controller peripheral.
    pub qspi: QSPI,
    /// Generic counter peripheral.
    pub counter: COUNTER,
    /// Exclusive APBC clock tokens.
    pub apbc_clocks: ApbcClocks,
    /// Application-processor power, clock, and reset peripheral.
    pub apmu: APMU,
    /// CPU configuration peripheral.
    pub ciu: CIU,
    /// Exclusive GPIO pad tokens.
    pub gpio: GpioPads,
    /// UART0 peripheral.
    pub uart0: UART0,
    /// UART1 peripheral.
    pub uart1: UART1,
    /// UART2 peripheral.
    pub uart2: UART2,
    /// UART3 peripheral.
    pub uart3: UART3,
    /// UART4 peripheral.
    pub uart4: UART4,
    /// UART5 peripheral.
    pub uart5: UART5,
    /// UART6 peripheral.
    pub uart6: UART6,
    /// UART7 peripheral.
    pub uart7: UART7,
    /// UART8 peripheral.
    pub uart8: UART8,
    /// UART9 peripheral.
    pub uart9: UART9,
    /// Real-time-domain UART0 peripheral.
    pub r_uart0: R_UART0,
    /// Real-time-domain UART1 peripheral.
    pub r_uart1: R_UART1,
    /// Secure I2C3 (TWSI3) peripheral.
    pub i2c3: I2C3,
    /// PWM0 peripheral.
    pub pwm0: PWM0,
    /// PWM1 peripheral.
    pub pwm1: PWM1,
    /// PWM2 peripheral.
    pub pwm2: PWM2,
    /// PWM3 peripheral.
    pub pwm3: PWM3,
    /// PWM4 peripheral.
    pub pwm4: PWM4,
    /// PWM5 peripheral.
    pub pwm5: PWM5,
    /// PWM6 peripheral.
    pub pwm6: PWM6,
    /// PWM7 peripheral.
    pub pwm7: PWM7,
    /// PWM8 peripheral.
    pub pwm8: PWM8,
    /// PWM9 peripheral.
    pub pwm9: PWM9,
    /// PWM10 peripheral.
    pub pwm10: PWM10,
    /// PWM11 peripheral.
    pub pwm11: PWM11,
    /// PWM12 peripheral.
    pub pwm12: PWM12,
    /// PWM13 peripheral.
    pub pwm13: PWM13,
    /// PWM14 peripheral.
    pub pwm14: PWM14,
    /// PWM15 peripheral.
    pub pwm15: PWM15,
    /// PWM16 peripheral.
    pub pwm16: PWM16,
    /// PWM17 peripheral.
    pub pwm17: PWM17,
    /// PWM18 peripheral.
    pub pwm18: PWM18,
    /// PWM19 peripheral.
    pub pwm19: PWM19,
    /// Secure SSP2 / SPI peripheral.
    pub spi2: SPI2,
    /// SSP3 / SPI peripheral.
    pub spi3: SPI3,
    /// Full-duplex I2S0 peripheral.
    pub i2s0: I2S0,
    /// Full-duplex I2S1 peripheral.
    pub i2s1: I2S1,
    /// SD / SDIO / eMMC host 0.
    pub sdh0: SDH0,
    /// SD / SDIO / eMMC host 1.
    pub sdh1: SDH1,
    /// SD / SDIO / eMMC host 2.
    pub sdh2: SDH2,
    /// Timer 1 and watchdog peripheral.
    pub timer1: TIMER1,
    /// Timer 2 and watchdog peripheral.
    pub timer2: TIMER2,
    /// Non-secure peripheral DMA controller.
    pub pdma: PDMA,
    /// Secure APB clock and reset controller.
    pub apbc2: APBC2,
    /// Real-time clock.
    pub rtc: RTC,
    /// Secure real-time clock.
    pub sec_rtc: SEC_RTC,
    /// Real-time CPU system controller.
    pub rcpu: RCPU,
    /// Real-time audio clock controller.
    pub r_audio_clock: R_AUDIO_CLOCK,
    /// Real-time power-management controller.
    pub rpmu: RPMU,
    /// Real-time PWM clock controller.
    pub r_pwm_clock: R_PWM_CLOCK,
    /// One-Wire bus master.
    pub onewire: ONEWIRE,
    /// Application-side mailbox.
    pub mailbox: MAILBOX,
    /// Real-time-side mailbox.
    pub r_mailbox: R_MAILBOX,
    /// Infrared receiver.
    pub ir: IR,
    /// Real-time infrared receiver.
    pub r_ir: R_IR,
    /// Temperature sensor.
    pub tsensor: TSENSOR,
    /// CAN-FD controller.
    pub can0: CAN0,
    /// Real-time CAN-FD controller.
    pub r_can0: R_CAN0,
    /// Random generator and DMA.
    pub trng: TRNG,
    /// Real-time AHB DMA.
    pub ahbdma: AHBDMA,
    /// Ethernet MAC 0.
    pub emac0: EMAC0,
    /// Ethernet MAC 1.
    pub emac1: EMAC1,
    /// PMU timer and watchdog.
    pub pmu_timer: PMU_TIMER,
    /// Secure timer and watchdog.
    pub sec_timer: SEC_TIMER,
    /// USB2 OTG controller.
    pub usb2_otg: USB2_OTG,
    /// USB2 host controller.
    pub usb2_host: USB2_HOST,
    /// USB2 OTG PHY.
    pub usb2_phy_otg: USB2_PHY_OTG,
    /// USB2 host PHY.
    pub usb2_phy_host: USB2_PHY_HOST,
    /// USB3 UTMI PHY.
    pub usb3_utmi: USB3_UTMI,
    /// DWC3 USB3 controller.
    pub usb3: USB3,
    /// Real-time PWM 0.
    pub r_pwm0: R_PWM0,
    /// Real-time PWM 1.
    pub r_pwm1: R_PWM1,
    /// Real-time PWM 2.
    pub r_pwm2: R_PWM2,
    /// Real-time PWM 3.
    pub r_pwm3: R_PWM3,
    /// Real-time PWM 4.
    pub r_pwm4: R_PWM4,
    /// Real-time PWM 5.
    pub r_pwm5: R_PWM5,
    /// Real-time PWM 6.
    pub r_pwm6: R_PWM6,
    /// Real-time PWM 7.
    pub r_pwm7: R_PWM7,
    /// Real-time PWM 8.
    pub r_pwm8: R_PWM8,
    /// Real-time PWM 9.
    pub r_pwm9: R_PWM9,
    /// Audio DMA 0.
    pub adma0: ADMA0,
    /// Half-duplex audio interface 0.
    pub ri2s0: RI2S0,
    /// Audio DMA 1.
    pub adma1: ADMA1,
    /// Half-duplex audio interface 1.
    pub ri2s1: RI2S1,
    /// Audio DMA 2.
    pub adma2: ADMA2,
    /// Half-duplex audio interface 2.
    pub ri2s2: RI2S2,
    /// Audio DMA 3.
    pub adma3: ADMA3,
    /// Half-duplex audio interface 3.
    pub ri2s3: RI2S3,
    /// PCIe 0 link-management registers.
    pub pcie0_link: PCIE0_LINK,
    /// PCIe 1 link-management registers.
    pub pcie1_link: PCIE1_LINK,
    /// PCIe 2 link-management registers.
    pub pcie2_link: PCIE2_LINK,
    /// DCIU peripheral.
    pub dciu: DCIU,
    /// DSI peripheral.
    pub dsi: DSI,
    /// 2D graphics engine.
    pub v2d: V2D,
    /// Camera capture and CSI PHY 0.
    pub ccic0: CCIC0,
    /// Camera capture and CSI PHY 1.
    pub ccic1: CCIC1,
    /// Camera capture and CSI PHY 2.
    pub ccic2: CCIC2,
    /// Video processing unit.
    pub vpu: VPU,
    /// Display processing unit 0.
    pub dpu0: DPU0,
    /// Display processing unit 1.
    pub dpu1: DPU1,
    /// Real-time I2C controller.
    pub r_i2c0: R_I2C0,
    /// Real-time SPI controller 0.
    pub r_spi0: R_SPI0,
    /// Platform interrupt controller.
    pub plic: PLIC,
}

impl Peripherals {
    /// Acquires peripheral tokens once across all harts and SoC modules.
    ///
    /// # Safety
    /// The hardware-access requirements of [`Self::steal`] must hold.
    #[inline]
    pub unsafe fn take() -> Option<Self> {
        if !super::claim_peripherals(&super::PERIPHERALS_TAKEN) {
            return None;
        }
        // SAFETY: the caller establishes hardware access; the atomic claims ownership.
        Some(unsafe { Self::steal() })
    }

    /// Acquires peripheral tokens without initializing hardware or checking ownership.
    ///
    /// # Safety
    ///
    /// Hart tokens must be unique, including after any previous spawn.
    ///
    /// Run on K1/M1 with aligned, identity-mapped registers accessible at the current
    /// privilege level, including secure and real-time peripherals.
    ///
    /// Retain valid power, upstream clocks and reset for every access,
    /// permanently for consumed tokens and pad/clock tokens.
    ///
    /// Stop conflicting users and DMA, including former pad users; no hart, firmware
    /// or duplicate owner may invalidate these guarantees, even after drop or forget.
    ///
    /// PLL sources must remain stable with a 24 MHz reference.
    ///
    /// Untracked clock consumers and DMA must tolerate shared-gate changes;
    /// HAL consumers must retain their controller borrows.
    ///
    /// Used pads must have valid electrical settings,
    /// and dedicated PWR_SCL/PWR_SDA must retain their I²C8 routing.
    ///
    /// The generic counter may be enabled; external writers must not reset it
    /// or change its value while borrowed or after its token is consumed.
    #[inline]
    pub unsafe fn steal() -> Self {
        super::PERIPHERALS_TAKEN.store(true, core::sync::atomic::Ordering::Release);
        Self {
            plic: PLIC {
                _private: core::marker::PhantomData,
            },
            vpu: VPU {
                _private: core::marker::PhantomData,
            },
            dpu0: DPU0 {
                _private: core::marker::PhantomData,
            },
            dpu1: DPU1 {
                _private: core::marker::PhantomData,
            },
            r_i2c0: R_I2C0 {
                _private: core::marker::PhantomData,
            },
            r_spi0: R_SPI0 {
                _private: core::marker::PhantomData,
            },
            v2d: V2D {
                _private: core::marker::PhantomData,
            },
            ccic0: CCIC0 {
                _private: core::marker::PhantomData,
            },
            ccic1: CCIC1 {
                _private: core::marker::PhantomData,
            },
            ccic2: CCIC2 {
                _private: core::marker::PhantomData,
            },
            dciu: DCIU {
                _private: core::marker::PhantomData,
            },
            dsi: DSI {
                _private: core::marker::PhantomData,
            },
            apbc2: APBC2 {
                _private: core::marker::PhantomData,
            },
            rtc: RTC {
                _private: core::marker::PhantomData,
            },
            sec_rtc: SEC_RTC {
                _private: core::marker::PhantomData,
            },
            rcpu: RCPU {
                _private: core::marker::PhantomData,
            },
            r_audio_clock: R_AUDIO_CLOCK {
                _private: core::marker::PhantomData,
            },
            rpmu: RPMU {
                _private: core::marker::PhantomData,
            },
            r_pwm_clock: R_PWM_CLOCK {
                _private: core::marker::PhantomData,
            },
            onewire: ONEWIRE {
                _private: core::marker::PhantomData,
            },
            mailbox: MAILBOX {
                _private: core::marker::PhantomData,
            },
            r_mailbox: R_MAILBOX {
                _private: core::marker::PhantomData,
            },
            ir: IR {
                _private: core::marker::PhantomData,
            },
            r_ir: R_IR {
                _private: core::marker::PhantomData,
            },
            tsensor: TSENSOR {
                _private: core::marker::PhantomData,
            },
            can0: CAN0 {
                _private: core::marker::PhantomData,
            },
            r_can0: R_CAN0 {
                _private: core::marker::PhantomData,
            },
            trng: TRNG {
                _private: core::marker::PhantomData,
            },
            ahbdma: AHBDMA {
                _private: core::marker::PhantomData,
            },
            emac0: EMAC0 {
                _private: core::marker::PhantomData,
            },
            emac1: EMAC1 {
                _private: core::marker::PhantomData,
            },
            pmu_timer: PMU_TIMER {
                _private: core::marker::PhantomData,
            },
            sec_timer: SEC_TIMER {
                _private: core::marker::PhantomData,
            },
            usb2_otg: USB2_OTG {
                _private: core::marker::PhantomData,
            },
            usb2_host: USB2_HOST {
                _private: core::marker::PhantomData,
            },
            usb2_phy_otg: USB2_PHY_OTG {
                _private: core::marker::PhantomData,
            },
            usb2_phy_host: USB2_PHY_HOST {
                _private: core::marker::PhantomData,
            },
            usb3_utmi: USB3_UTMI {
                _private: core::marker::PhantomData,
            },
            usb3: USB3 {
                _private: core::marker::PhantomData,
            },
            r_pwm0: R_PWM0 {
                _private: core::marker::PhantomData,
            },
            r_pwm1: R_PWM1 {
                _private: core::marker::PhantomData,
            },
            r_pwm2: R_PWM2 {
                _private: core::marker::PhantomData,
            },
            r_pwm3: R_PWM3 {
                _private: core::marker::PhantomData,
            },
            r_pwm4: R_PWM4 {
                _private: core::marker::PhantomData,
            },
            r_pwm5: R_PWM5 {
                _private: core::marker::PhantomData,
            },
            r_pwm6: R_PWM6 {
                _private: core::marker::PhantomData,
            },
            r_pwm7: R_PWM7 {
                _private: core::marker::PhantomData,
            },
            r_pwm8: R_PWM8 {
                _private: core::marker::PhantomData,
            },
            r_pwm9: R_PWM9 {
                _private: core::marker::PhantomData,
            },
            adma0: ADMA0 {
                _private: core::marker::PhantomData,
            },
            ri2s0: RI2S0 {
                _private: core::marker::PhantomData,
            },
            adma1: ADMA1 {
                _private: core::marker::PhantomData,
            },
            ri2s1: RI2S1 {
                _private: core::marker::PhantomData,
            },
            adma2: ADMA2 {
                _private: core::marker::PhantomData,
            },
            ri2s2: RI2S2 {
                _private: core::marker::PhantomData,
            },
            adma3: ADMA3 {
                _private: core::marker::PhantomData,
            },
            ri2s3: RI2S3 {
                _private: core::marker::PhantomData,
            },
            pcie0_link: PCIE0_LINK {
                _private: core::marker::PhantomData,
            },
            pcie1_link: PCIE1_LINK {
                _private: core::marker::PhantomData,
            },
            pcie2_link: PCIE2_LINK {
                _private: core::marker::PhantomData,
            },
            // SAFETY: These are all K1 application hart IDs; steal transfers them once.
            harts: Harts {
                hart0: unsafe { crate::hart::Hart::new() },
                hart1: unsafe { crate::hart::Hart::new() },
                hart2: unsafe { crate::hart::Hart::new() },
                hart3: unsafe { crate::hart::Hart::new() },
                hart4: unsafe { crate::hart::Hart::new() },
                hart5: unsafe { crate::hart::Hart::new() },
                hart6: unsafe { crate::hart::Hart::new() },
                hart7: unsafe { crate::hart::Hart::new() },
            },
            pmic_pads: PmicPads {
                _private: core::marker::PhantomData,
            },
            i2c0: I2C0 {
                _private: core::marker::PhantomData,
            },
            i2c1: I2C1 {
                _private: core::marker::PhantomData,
            },
            i2c2: I2C2 {
                _private: core::marker::PhantomData,
            },
            i2c4: I2C4 {
                _private: core::marker::PhantomData,
            },
            i2c5: I2C5 {
                _private: core::marker::PhantomData,
            },
            i2c6: I2C6 {
                _private: core::marker::PhantomData,
            },
            i2c7: I2C7 {
                _private: core::marker::PhantomData,
            },
            i2c8: I2C8 {
                _private: core::marker::PhantomData,
            },
            apbs: APBS {
                _private: core::marker::PhantomData,
            },
            mpmu: MPMU {
                _private: core::marker::PhantomData,
            },
            qspi: QSPI {
                _private: core::marker::PhantomData,
            },
            counter: COUNTER {
                _private: core::marker::PhantomData,
            },
            // SAFETY: The caller transfers exclusive, permanently mapped APBC access.
            apbc_clocks: unsafe { ApbcClocks::new() },
            apmu: APMU {
                _private: core::marker::PhantomData,
            },
            ciu: CIU {
                _private: core::marker::PhantomData,
            },
            // SAFETY: The caller transfers all GPIO bits and MFPR registers once.
            gpio: unsafe { GpioPads::new() },
            uart0: UART0 {
                _private: core::marker::PhantomData,
            },
            uart1: UART1 {
                _private: core::marker::PhantomData,
            },
            uart2: UART2 {
                _private: core::marker::PhantomData,
            },
            uart3: UART3 {
                _private: core::marker::PhantomData,
            },
            uart4: UART4 {
                _private: core::marker::PhantomData,
            },
            uart5: UART5 {
                _private: core::marker::PhantomData,
            },
            uart6: UART6 {
                _private: core::marker::PhantomData,
            },
            uart7: UART7 {
                _private: core::marker::PhantomData,
            },
            uart8: UART8 {
                _private: core::marker::PhantomData,
            },
            uart9: UART9 {
                _private: core::marker::PhantomData,
            },
            r_uart0: R_UART0 {
                _private: core::marker::PhantomData,
            },
            r_uart1: R_UART1 {
                _private: core::marker::PhantomData,
            },
            i2c3: I2C3 {
                _private: core::marker::PhantomData,
            },
            pwm0: PWM0 {
                _private: core::marker::PhantomData,
            },
            pwm1: PWM1 {
                _private: core::marker::PhantomData,
            },
            pwm2: PWM2 {
                _private: core::marker::PhantomData,
            },
            pwm3: PWM3 {
                _private: core::marker::PhantomData,
            },
            pwm4: PWM4 {
                _private: core::marker::PhantomData,
            },
            pwm5: PWM5 {
                _private: core::marker::PhantomData,
            },
            pwm6: PWM6 {
                _private: core::marker::PhantomData,
            },
            pwm7: PWM7 {
                _private: core::marker::PhantomData,
            },
            pwm8: PWM8 {
                _private: core::marker::PhantomData,
            },
            pwm9: PWM9 {
                _private: core::marker::PhantomData,
            },
            pwm10: PWM10 {
                _private: core::marker::PhantomData,
            },
            pwm11: PWM11 {
                _private: core::marker::PhantomData,
            },
            pwm12: PWM12 {
                _private: core::marker::PhantomData,
            },
            pwm13: PWM13 {
                _private: core::marker::PhantomData,
            },
            pwm14: PWM14 {
                _private: core::marker::PhantomData,
            },
            pwm15: PWM15 {
                _private: core::marker::PhantomData,
            },
            pwm16: PWM16 {
                _private: core::marker::PhantomData,
            },
            pwm17: PWM17 {
                _private: core::marker::PhantomData,
            },
            pwm18: PWM18 {
                _private: core::marker::PhantomData,
            },
            pwm19: PWM19 {
                _private: core::marker::PhantomData,
            },
            spi2: SPI2 {
                _private: core::marker::PhantomData,
            },
            spi3: SPI3 {
                _private: core::marker::PhantomData,
            },
            i2s0: I2S0 {
                _private: core::marker::PhantomData,
            },
            i2s1: I2S1 {
                _private: core::marker::PhantomData,
            },
            sdh0: SDH0 {
                _private: core::marker::PhantomData,
            },
            sdh1: SDH1 {
                _private: core::marker::PhantomData,
            },
            sdh2: SDH2 {
                _private: core::marker::PhantomData,
            },
            timer1: TIMER1 {
                _private: core::marker::PhantomData,
            },
            timer2: TIMER2 {
                _private: core::marker::PhantomData,
            },
            pdma: PDMA {
                _private: core::marker::PhantomData,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Deref;

    #[test]
    fn peripheral_addresses_and_types() {
        fn register_type<T: Deref<Target = R> + AsRef<R>, R>() {}
        register_type::<I2C0, i2c::RegisterBlock>();
        assert_eq!(I2C0::ptr() as usize, 0xd401_0800);
        register_type::<I2C1, i2c::RegisterBlock>();
        assert_eq!(I2C1::ptr() as usize, 0xd401_1000);
        register_type::<I2C2, i2c::RegisterBlock>();
        assert_eq!(I2C2::ptr() as usize, 0xd401_2000);
        register_type::<I2C4, i2c::RegisterBlock>();
        assert_eq!(I2C4::ptr() as usize, 0xd401_2800);
        register_type::<I2C5, i2c::RegisterBlock>();
        assert_eq!(I2C5::ptr() as usize, 0xd401_3800);
        register_type::<I2C6, i2c::RegisterBlock>();
        assert_eq!(I2C6::ptr() as usize, 0xd401_8800);
        register_type::<I2C7, i2c::RegisterBlock>();
        assert_eq!(I2C7::ptr() as usize, 0xd401_d000);
        register_type::<I2C8, i2c::RegisterBlock>();
        assert_eq!(I2C8::ptr() as usize, 0xd401_d800);
        register_type::<APBS, apbs::k1::RegisterBlock>();
        assert_eq!(APBS::ptr() as usize, 0xd409_0000);
        register_type::<MPMU, mpmu::k1::RegisterBlock>();
        fn controller_types<A, M>()
        where
            A: apbs::Instance<'static, RegisterBlock = apbs::k1::RegisterBlock>,
            M: mpmu::Instance<'static, RegisterBlock = mpmu::k1::RegisterBlock>,
            for<'a> &'a mut A: apbs::Instance<'a, RegisterBlock = apbs::k1::RegisterBlock>,
            for<'a> &'a mut M: mpmu::Instance<'a, RegisterBlock = mpmu::k1::RegisterBlock>,
        {
        }
        controller_types::<APBS, MPMU>();
        assert_eq!(MPMU::ptr() as usize, 0xd405_0000);
        register_type::<MFPR, mfpr::k1::RegisterBlock>();
        assert_eq!(MFPR::ptr() as usize, 0xd401_e000);
        register_type::<QSPI, qspi::RegisterBlock>();
        fn qspi_type<T: qspi::Instance<'static>>()
        where
            for<'a> &'a mut T: qspi::Instance<'a>,
        {
        }
        qspi_type::<QSPI>();
        assert_eq!(QSPI::ptr() as usize, 0xd420_c000);
        register_type::<COUNTER, counter::k1::RegisterBlock>();
        fn counter_type<T: counter::Instance<'static>>()
        where
            for<'a> &'a mut T: counter::Instance<'a>,
        {
        }
        counter_type::<COUNTER>();
        assert_eq!(COUNTER::ptr() as usize, 0xd500_1000);
        assert_eq!(
            APBS::ptr() as usize
                + core::mem::offset_of!(apbs::k1::RegisterBlock, pll1_software_control2),
            0xd409_0104
        );
        assert_eq!(
            MPMU::ptr() as usize
                + core::mem::offset_of!(mpmu::k1::RegisterBlock, application_clock_gate),
            0xd405_1024
        );
        fn apmu_type<
            T: Deref<Target = apmu::k1::RegisterBlock> + AsRef<apmu::k1::RegisterBlock>,
        >() {
        }
        fn gpio_type<
            T: Deref<Target = gpio::k1::RegisterBlock> + AsRef<gpio::k1::RegisterBlock>,
        >() {
        }
        fn uart_type<
            T: Deref<Target = uart::RegisterBlock>
                + AsRef<uart::RegisterBlock>
                + uart::Instance<'static>,
        >()
        where
            for<'a> &'a mut T: uart::Instance<'a>,
        {
        }

        apmu_type::<APMU>();
        gpio_type::<GPIO>();
        uart_type::<UART0>();
        uart_type::<UART1>();
        uart_type::<UART2>();
        uart_type::<UART3>();
        uart_type::<UART4>();
        uart_type::<UART5>();
        uart_type::<UART6>();
        uart_type::<UART7>();
        uart_type::<UART8>();
        uart_type::<UART9>();
        uart_type::<R_UART0>();
        uart_type::<R_UART1>();
        register_type::<APBC, apbc::k1::RegisterBlock>();
        assert_eq!(APBC::ptr() as usize, 0xd401_5000);
        assert_eq!(APMU::ptr() as usize, 0xd428_2800);
        register_type::<CIU, ciu::k1::RegisterBlock>();
        assert_eq!(CIU::ptr() as usize, 0xd428_2c00);
        assert_eq!(
            APMU::ptr() as usize + core::mem::offset_of!(apmu::k1::RegisterBlock, qspi_clock_reset),
            0xd428_2860,
        );
        assert_eq!(GPIO::ptr() as usize, 0xd401_9000);
        assert_eq!(UART0::ptr() as usize, 0xd401_7000);
        assert_eq!(UART1::ptr() as usize, 0xf061_2000);
        assert_eq!(UART2::ptr() as usize, 0xd401_7100);
        assert_eq!(UART3::ptr() as usize, 0xd401_7200);
        assert_eq!(UART4::ptr() as usize, 0xd401_7300);
        assert_eq!(UART5::ptr() as usize, 0xd401_7400);
        assert_eq!(UART6::ptr() as usize, 0xd401_7500);
        assert_eq!(UART7::ptr() as usize, 0xd401_7600);
        assert_eq!(UART8::ptr() as usize, 0xd401_7700);
        assert_eq!(UART9::ptr() as usize, 0xd401_7800);
        assert_eq!(R_UART0::ptr() as usize, 0xc088_1000);
        assert_eq!(R_UART1::ptr() as usize, 0xc088_d000);
        fn address<T: Deref<Target = R> + AsRef<R>, R>(ptr: *const R, expected: usize) {
            assert_eq!(ptr as usize, expected);
        }
        address::<I2C3, i2c::RegisterBlock>(I2C3::ptr(), 0xf061_4000);
        address::<PWM0, pwm::k1::RegisterBlock>(PWM0::ptr(), 0xd401_a000);
        address::<PWM1, pwm::k1::RegisterBlock>(PWM1::ptr(), 0xd401_a400);
        address::<PWM2, pwm::k1::RegisterBlock>(PWM2::ptr(), 0xd401_a800);
        address::<PWM3, pwm::k1::RegisterBlock>(PWM3::ptr(), 0xd401_ac00);
        address::<PWM4, pwm::k1::RegisterBlock>(PWM4::ptr(), 0xd401_b000);
        address::<PWM5, pwm::k1::RegisterBlock>(PWM5::ptr(), 0xd401_b400);
        address::<PWM6, pwm::k1::RegisterBlock>(PWM6::ptr(), 0xd401_b800);
        address::<PWM7, pwm::k1::RegisterBlock>(PWM7::ptr(), 0xd401_bc00);
        address::<PWM8, pwm::k1::RegisterBlock>(PWM8::ptr(), 0xd402_0000);
        address::<PWM9, pwm::k1::RegisterBlock>(PWM9::ptr(), 0xd402_0400);
        address::<PWM10, pwm::k1::RegisterBlock>(PWM10::ptr(), 0xd402_0800);
        address::<PWM11, pwm::k1::RegisterBlock>(PWM11::ptr(), 0xd402_0c00);
        address::<PWM12, pwm::k1::RegisterBlock>(PWM12::ptr(), 0xd402_1000);
        address::<PWM13, pwm::k1::RegisterBlock>(PWM13::ptr(), 0xd402_1400);
        address::<PWM14, pwm::k1::RegisterBlock>(PWM14::ptr(), 0xd402_1800);
        address::<PWM15, pwm::k1::RegisterBlock>(PWM15::ptr(), 0xd402_1c00);
        address::<PWM16, pwm::k1::RegisterBlock>(PWM16::ptr(), 0xd402_2000);
        address::<PWM17, pwm::k1::RegisterBlock>(PWM17::ptr(), 0xd402_2400);
        address::<PWM18, pwm::k1::RegisterBlock>(PWM18::ptr(), 0xd402_2800);
        address::<PWM19, pwm::k1::RegisterBlock>(PWM19::ptr(), 0xd402_2c00);
        address::<SPI2, spi::k1::RegisterBlock>(SPI2::ptr(), 0xf061_3000);
        address::<SPI3, spi::k1::RegisterBlock>(SPI3::ptr(), 0xd401_c000);
        address::<I2S0, spi::k1::RegisterBlock>(I2S0::ptr(), 0xd402_6000);
        address::<I2S1, spi::k1::RegisterBlock>(I2S1::ptr(), 0xd402_6800);
        address::<SDH0, sdh::k1::RegisterBlock>(SDH0::ptr(), 0xd428_0000);
        address::<SDH1, sdh::k1::RegisterBlock>(SDH1::ptr(), 0xd428_0800);
        address::<SDH2, sdh::k1::RegisterBlock>(SDH2::ptr(), 0xd428_1000);
        address::<TIMER1, timer::k1::RegisterBlock>(TIMER1::ptr(), 0xd401_4000);
        address::<TIMER2, timer::k1::RegisterBlock>(TIMER2::ptr(), 0xd401_6000);
        address::<PDMA, pdma::k1::RegisterBlock>(PDMA::ptr(), 0xd400_0000);
        address::<APBC2, apbc2::k1::RegisterBlock>(APBC2::ptr(), 0xf061_0000);
        address::<RTC, rtc::k1::RegisterBlock>(RTC::ptr(), 0xd401_0000);
        address::<SEC_RTC, rtc::k1::RegisterBlock>(SEC_RTC::ptr(), 0xf061_5000);
        address::<RCPU, rcpu::k1::RegisterBlock>(RCPU::ptr(), 0xc088_0000);
        address::<R_AUDIO_CLOCK, rcpu::k1::AudioClockRegisters>(R_AUDIO_CLOCK::ptr(), 0xc088_2000);
        address::<RPMU, rpmu::k1::RegisterBlock>(RPMU::ptr(), 0xc088_c000);
        address::<R_PWM_CLOCK, rcpu::k1::PwmClockRegisters>(R_PWM_CLOCK::ptr(), 0xc088_8000);
        address::<ONEWIRE, onewire::k1::RegisterBlock>(ONEWIRE::ptr(), 0xd401_1800);
        address::<MAILBOX, mailbox::k1::RegisterBlock>(MAILBOX::ptr(), 0xd401_3400);
        address::<R_MAILBOX, mailbox::k1::RegisterBlock>(R_MAILBOX::ptr(), 0xc088_a000);
        address::<IR, ir::k1::RegisterBlock>(IR::ptr(), 0xd401_7f00);
        address::<R_IR, ir::k1::RegisterBlock>(R_IR::ptr(), 0xc088_e000);
        address::<TSENSOR, tsensor::k1::RegisterBlock>(TSENSOR::ptr(), 0xd401_8000);
        address::<CAN0, can::k1::RegisterBlock>(CAN0::ptr(), 0xd402_8000);
        address::<R_CAN0, can::k1::RegisterBlock>(R_CAN0::ptr(), 0xc087_0000);
        address::<TRNG, trng::k1::RegisterBlock>(TRNG::ptr(), 0xf070_3800);
        address::<AHBDMA, ahbdma::k1::RegisterBlock>(AHBDMA::ptr(), 0xc088_4000);
        address::<EMAC0, emac::k1::RegisterBlock>(EMAC0::ptr(), 0xcac8_0000);
        address::<EMAC1, emac::k1::RegisterBlock>(EMAC1::ptr(), 0xcac8_1000);
        address::<PMU_TIMER, timer::k1::RegisterBlock>(PMU_TIMER::ptr(), 0xd408_0000);
        address::<SEC_TIMER, timer::k1::RegisterBlock>(SEC_TIMER::ptr(), 0xf061_6000);
        address::<USB2_OTG, usb2::k1::RegisterBlock>(USB2_OTG::ptr(), 0xc090_0100);
        address::<USB2_HOST, usb2::k1::RegisterBlock>(USB2_HOST::ptr(), 0xc098_0100);
        address::<USB2_PHY_OTG, usb2_phy::k1::RegisterBlock>(USB2_PHY_OTG::ptr(), 0xc094_0000);
        address::<USB2_PHY_HOST, usb2_phy::k1::RegisterBlock>(USB2_PHY_HOST::ptr(), 0xc09c_0000);
        address::<USB3_UTMI, usb2_phy::k1::RegisterBlock>(USB3_UTMI::ptr(), 0xc0a3_0000);
        address::<USB3, usb3::RegisterBlock>(USB3::ptr(), 0xc0a0_0000);
        address::<R_PWM0, pwm::k1::RegisterBlock>(R_PWM0::ptr(), 0xc088_8100);
        address::<R_PWM1, pwm::k1::RegisterBlock>(R_PWM1::ptr(), 0xc088_8200);
        address::<R_PWM2, pwm::k1::RegisterBlock>(R_PWM2::ptr(), 0xc088_8300);
        address::<R_PWM3, pwm::k1::RegisterBlock>(R_PWM3::ptr(), 0xc088_8400);
        address::<R_PWM4, pwm::k1::RegisterBlock>(R_PWM4::ptr(), 0xc088_8500);
        address::<R_PWM5, pwm::k1::RegisterBlock>(R_PWM5::ptr(), 0xc088_8600);
        address::<R_PWM6, pwm::k1::RegisterBlock>(R_PWM6::ptr(), 0xc088_8700);
        address::<R_PWM7, pwm::k1::RegisterBlock>(R_PWM7::ptr(), 0xc088_8800);
        address::<R_PWM8, pwm::k1::RegisterBlock>(R_PWM8::ptr(), 0xc088_8900);
        address::<R_PWM9, pwm::k1::RegisterBlock>(R_PWM9::ptr(), 0xc088_8a00);
        address::<ADMA0, adma::k1::RegisterBlock>(ADMA0::ptr(), 0xc088_3000);
        address::<RI2S0, ri2s::k1::RegisterBlock>(RI2S0::ptr(), 0xc088_3100);
        address::<ADMA1, adma::k1::RegisterBlock>(ADMA1::ptr(), 0xc088_3400);
        address::<RI2S1, ri2s::k1::RegisterBlock>(RI2S1::ptr(), 0xc088_3500);
        address::<ADMA2, adma::k1::RegisterBlock>(ADMA2::ptr(), 0xc088_3800);
        address::<RI2S2, ri2s::k1::RegisterBlock>(RI2S2::ptr(), 0xc088_3900);
        address::<ADMA3, adma::k1::RegisterBlock>(ADMA3::ptr(), 0xc088_3c00);
        address::<RI2S3, ri2s::k1::RegisterBlock>(RI2S3::ptr(), 0xc088_3d00);
        address::<PCIE0_LINK, pcie::k1::RegisterBlock>(PCIE0_LINK::ptr(), 0xc0b2_0000);
        address::<PCIE1_LINK, pcie::k1::RegisterBlock>(PCIE1_LINK::ptr(), 0xc0c2_0000);
        address::<PCIE2_LINK, pcie::k1::RegisterBlock>(PCIE2_LINK::ptr(), 0xc0d2_0000);
        address::<DCIU, dciu::k1::RegisterBlock>(DCIU::ptr(), 0xd844_0000);
        address::<DSI, dsi::k1::RegisterBlock>(DSI::ptr(), 0xd421_a800);
        address::<V2D, v2d::RegisterBlock>(V2D::ptr(), 0xc010_0000);
        address::<CCIC0, ccic::k1::RegisterBlock>(CCIC0::ptr(), 0xd420_a000);
        address::<CCIC1, ccic::k1::RegisterBlock>(CCIC1::ptr(), 0xd420_a800);
        address::<CCIC2, ccic::k1::RegisterBlock>(CCIC2::ptr(), 0xd420_6000);
        address::<VPU, vpu::RegisterBlock>(VPU::ptr(), 0xc050_0000);
        address::<DPU0, dpu::k1::RegisterBlock>(DPU0::ptr(), 0xc034_0000);
        address::<DPU1, dpu::k1::RegisterBlock>(DPU1::ptr(), 0xc044_0000);
        address::<R_I2C0, i2c::RegisterBlock>(R_I2C0::ptr(), 0xc088_7000);
        address::<R_SPI0, spi::k1::RegisterBlock>(R_SPI0::ptr(), 0xc088_5000);
        address::<PLIC, plic::k1::RegisterBlock>(PLIC::ptr(), 0xe000_0000);
        assert_eq!(core::mem::size_of::<Peripherals>(), 0);
    }
}
