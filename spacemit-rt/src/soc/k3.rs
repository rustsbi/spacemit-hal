//! K3 peripheral ownership and addresses.

use spacemit_hal::{
    adma, apbc, apbs, apmu, can, ciu, espi, gpio, hdma, hsio_phy, i2c, iopmp, ir, mailbox, mfpr,
    mpmu, pdma, pwm, qspi, ri2s, sdh, sec_ciu, spi, spinlock, timer, tsensor, uart, ufs, usb2_phy,
};

// Address map: K3 manual and /proc/device-tree on K3 Pico ITX, read 2026-09-08.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/06_address_map.md
// The running /soc has two address/size cells and empty ranges (physical addresses).
// /proc/device-tree/aliases/serial1 selects /soc/serial@f0612000.
// R_UART0..5 name the real-time-domain nodes (spacemit,k1-uart), whose
// serial11..16 aliases select /soc/serial@c0881000 through serial@c0881500.
// SDH0..2 use the software instance IDs, not MMC card enumeration.

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
    /// I2C8 (TWSI8) peripheral.
    pub struct I2C8 => 0xd401_d800, i2c::RegisterBlock;
    /// PLL clock-control peripheral.
    pub struct APBS => 0xd409_0000, apbs::k3::RegisterBlock;
    /// Main power-management peripheral.
    pub struct MPMU => 0xd405_0000, mpmu::k3::RegisterBlock;
    /// Multi-function pad peripheral.
    pub struct MFPR => 0xd401_e000, mfpr::k3::RegisterBlock;
    /// Quad-SPI memory-controller peripheral.
    pub struct QSPI => 0xd420_c000, qspi::RegisterBlock;
    /// APB clock and reset peripheral.
    pub struct APBC => 0xd401_5000, apbc::k3::RegisterBlock;
    /// Application-processor power, clock, and reset peripheral.
    pub struct APMU => 0xd428_2800, apmu::k3::RegisterBlock;
    /// CPU configuration peripheral.
    pub struct CIU => 0xd428_2c00, ciu::k3::RegisterBlock;
    /// GPIO peripheral.
    pub struct GPIO => 0xd401_9000, gpio::k3::RegisterBlock;
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
    /// UART10 peripheral.
    pub struct UART10 => 0xd401_f000, uart::RegisterBlock;
    /// Real-time-domain UART0 peripheral.
    pub struct R_UART0 => 0xc088_1000, uart::RegisterBlock;
    /// Real-time-domain UART1 peripheral.
    pub struct R_UART1 => 0xc088_1100, uart::RegisterBlock;
    /// Real-time-domain UART2 peripheral.
    pub struct R_UART2 => 0xc088_1200, uart::RegisterBlock;
    /// Real-time-domain UART3 peripheral.
    pub struct R_UART3 => 0xc088_1300, uart::RegisterBlock;
    /// Real-time-domain UART4 peripheral.
    pub struct R_UART4 => 0xc088_1400, uart::RegisterBlock;
    /// Real-time-domain UART5 peripheral.
    pub struct R_UART5 => 0xc088_1500, uart::RegisterBlock;

    /// Secure I2C3 (TWSI3) peripheral.
    pub struct I2C3 => 0xf061_4000, i2c::RegisterBlock;
    /// PWM0 peripheral.
    pub struct PWM0 => 0xd401_a000, pwm::k3::RegisterBlock;
    /// PWM1 peripheral.
    pub struct PWM1 => 0xd401_a400, pwm::k3::RegisterBlock;
    /// PWM2 peripheral.
    pub struct PWM2 => 0xd401_a800, pwm::k3::RegisterBlock;
    /// PWM3 peripheral.
    pub struct PWM3 => 0xd401_ac00, pwm::k3::RegisterBlock;
    /// PWM4 peripheral.
    pub struct PWM4 => 0xd401_b000, pwm::k3::RegisterBlock;
    /// PWM5 peripheral.
    pub struct PWM5 => 0xd401_b400, pwm::k3::RegisterBlock;
    /// PWM6 peripheral.
    pub struct PWM6 => 0xd401_b800, pwm::k3::RegisterBlock;
    /// PWM7 peripheral.
    pub struct PWM7 => 0xd401_bc00, pwm::k3::RegisterBlock;
    /// PWM8 peripheral.
    pub struct PWM8 => 0xd402_0000, pwm::k3::RegisterBlock;
    /// PWM9 peripheral.
    pub struct PWM9 => 0xd402_0400, pwm::k3::RegisterBlock;
    /// PWM10 peripheral.
    pub struct PWM10 => 0xd402_0800, pwm::k3::RegisterBlock;
    /// PWM11 peripheral.
    pub struct PWM11 => 0xd402_0c00, pwm::k3::RegisterBlock;
    /// PWM12 peripheral.
    pub struct PWM12 => 0xd402_1000, pwm::k3::RegisterBlock;
    /// PWM13 peripheral.
    pub struct PWM13 => 0xd402_1400, pwm::k3::RegisterBlock;
    /// PWM14 peripheral.
    pub struct PWM14 => 0xd402_1800, pwm::k3::RegisterBlock;
    /// PWM15 peripheral.
    pub struct PWM15 => 0xd402_1c00, pwm::k3::RegisterBlock;
    /// PWM16 peripheral.
    pub struct PWM16 => 0xd402_2000, pwm::k3::RegisterBlock;
    /// PWM17 peripheral.
    pub struct PWM17 => 0xd402_2400, pwm::k3::RegisterBlock;
    /// PWM18 peripheral.
    pub struct PWM18 => 0xd402_2800, pwm::k3::RegisterBlock;
    /// PWM19 peripheral.
    pub struct PWM19 => 0xd402_2c00, pwm::k3::RegisterBlock;
    /// Secure SSP2 / SPI peripheral.
    pub struct SPI2 => 0xf061_3000, spi::k3::RegisterBlock;
    /// SSP3 / SPI peripheral.
    pub struct SPI3 => 0xd401_c000, spi::k3::RegisterBlock;
    /// Full-duplex I2S0 peripheral.
    pub struct I2S0 => 0xd402_6000, spi::k3::RegisterBlock;
    /// Full-duplex I2S1 peripheral.
    pub struct I2S1 => 0xd402_6800, spi::k3::RegisterBlock;
    /// Full-duplex I2S2 peripheral.
    pub struct I2S2 => 0xd402_7000, spi::k3::RegisterBlock;
    /// Full-duplex I2S3 peripheral.
    pub struct I2S3 => 0xd402_7800, spi::k3::RegisterBlock;
    /// Full-duplex I2S4 peripheral.
    pub struct I2S4 => 0xd404_1000, spi::k3::RegisterBlock;
    /// Full-duplex I2S5 peripheral.
    pub struct I2S5 => 0xd404_1800, spi::k3::RegisterBlock;
    /// SD / SDIO / eMMC host 0.
    pub struct SDH0 => 0xd428_0000, sdh::k3::RegisterBlock;
    /// SD / SDIO / eMMC host 1.
    pub struct SDH1 => 0xd428_0800, sdh::k3::RegisterBlock;
    /// SD / SDIO / eMMC host 2.
    pub struct SDH2 => 0xd428_1000, sdh::k3::RegisterBlock;
    /// Timer 0 and watchdog peripheral.
    pub struct TIMER0 => 0xd401_4000, timer::k3::RegisterBlock;
    /// Timer 1 and watchdog peripheral.
    pub struct TIMER1 => 0xd401_6000, timer::k3::RegisterBlock;
    /// Non-secure peripheral DMA controller.
    pub struct PDMA => 0xd400_0000, pdma::k3::RegisterBlock;
    /// Real-time-domain I2C0 peripheral.
    pub struct R_I2C0 => 0xc088_6000, i2c::RegisterBlock;
    /// Real-time-domain I2C1 peripheral.
    pub struct R_I2C1 => 0xc088_6100, i2c::RegisterBlock;
    // RCPU PWM uses pwm-pxa.c's three-word interface; output-count support is unverified.
    /// Real-time-domain PWM0 legacy control/duty/period registers.
    pub struct R_PWM0 => 0xc088_d100, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM1 legacy control/duty/period registers.
    pub struct R_PWM1 => 0xc088_d200, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM2 legacy control/duty/period registers.
    pub struct R_PWM2 => 0xc088_d300, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM3 legacy control/duty/period registers.
    pub struct R_PWM3 => 0xc088_d400, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM4 legacy control/duty/period registers.
    pub struct R_PWM4 => 0xc088_d500, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM5 legacy control/duty/period registers.
    pub struct R_PWM5 => 0xc088_d600, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM6 legacy control/duty/period registers.
    pub struct R_PWM6 => 0xc088_d700, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM7 legacy control/duty/period registers.
    pub struct R_PWM7 => 0xc088_d800, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM8 legacy control/duty/period registers.
    pub struct R_PWM8 => 0xc088_d900, pwm::k1::RegisterBlock;
    /// Real-time-domain PWM9 legacy control/duty/period registers.
    pub struct R_PWM9 => 0xc088_da00, pwm::k1::RegisterBlock;
    /// SSP0 / SPI peripheral.
    pub struct SPI0 => 0xd404_0000, spi::k3::RegisterBlock;
    /// SSP1 / SPI peripheral.
    pub struct SPI1 => 0xd404_0800, spi::k3::RegisterBlock;
    /// Real-time-domain SSP0 / SPI peripheral.
    pub struct R_SPI0 => 0xc088_5000, spi::k3::RegisterBlock;
    /// Real-time-domain SSP1 / SPI peripheral.
    pub struct R_SPI1 => 0xc088_5100, spi::k3::RegisterBlock;
    /// Real-time-domain SSP2 / SPI peripheral.
    pub struct R_SPI2 => 0xc088_5200, spi::k3::RegisterBlock;
    /// PMU timer and watchdog peripheral.
    pub struct PMU_TIMER => 0xd408_0000, timer::k3::RegisterBlock;
    /// Secure timer 8 and watchdog peripheral.
    pub struct SEC_TIMER => 0xf061_6000, timer::k3::RegisterBlock;
    // Live hdma0..7 nodes use spacemit-ai-dmac; ai_dma_ctrl.c matches this layout.
    // https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/dma/ai_dma_ctrl.c
    /// HDMA0 (AIDMA0) controller.
    pub struct HDMA0 => 0xd880_4000, hdma::k3::RegisterBlock;
    /// HDMA1 (AIDMA1) controller.
    pub struct HDMA1 => 0xd880_5000, hdma::k3::RegisterBlock;
    /// HDMA2 (AIDMA2) controller.
    pub struct HDMA2 => 0xd880_6000, hdma::k3::RegisterBlock;
    /// HDMA3 (AIDMA3) controller.
    pub struct HDMA3 => 0xd880_7000, hdma::k3::RegisterBlock;
    /// HDMA4 (AIDMA4) controller.
    pub struct HDMA4 => 0xd880_8000, hdma::k3::RegisterBlock;
    /// HDMA5 (AIDMA5) controller.
    pub struct HDMA5 => 0xd880_9000, hdma::k3::RegisterBlock;
    /// HDMA6 (AIDMA6) controller.
    pub struct HDMA6 => 0xd880_a000, hdma::k3::RegisterBlock;
    /// HDMA7 (AIDMA7) controller.
    pub struct HDMA7 => 0xd880_b000, hdma::k3::RegisterBlock;
    /// CAN-FD0 peripheral.
    pub struct CAN0 => 0xd402_8000, can::k3::RegisterBlock;
    /// CAN-FD1 peripheral.
    pub struct CAN1 => 0xd402_c000, can::k3::RegisterBlock;
    /// CAN-FD2 peripheral.
    pub struct CAN2 => 0xd403_4000, can::k3::RegisterBlock;
    /// CAN-FD3 peripheral.
    pub struct CAN3 => 0xd403_8000, can::k3::RegisterBlock;
    /// CAN-FD4 peripheral.
    pub struct CAN4 => 0xd403_c000, can::k3::RegisterBlock;
    /// Real-time-domain CAN-FD0 peripheral.
    pub struct R_CAN0 => 0xc071_0000, can::k3::RegisterBlock;
    /// Real-time-domain CAN-FD1 peripheral.
    pub struct R_CAN1 => 0xc072_0000, can::k3::RegisterBlock;
    /// Real-time-domain CAN-FD2 peripheral.
    pub struct R_CAN2 => 0xc073_0000, can::k3::RegisterBlock;
    /// Real-time-domain CAN-FD3 peripheral.
    pub struct R_CAN3 => 0xc074_0000, can::k3::RegisterBlock;
    /// Real-time-domain CAN-FD4 peripheral.
    pub struct R_CAN4 => 0xc075_0000, can::k3::RegisterBlock;
    /// Audio DMA0 controller.
    pub struct ADMA0 => 0xc088_3000, adma::k3::RegisterBlock;
    /// Audio DMA1 controller.
    pub struct ADMA1 => 0xc088_3400, adma::k3::RegisterBlock;
    /// Audio DMA2 controller.
    pub struct ADMA2 => 0xc088_3800, adma::k3::RegisterBlock;
    /// Audio DMA3 controller.
    pub struct ADMA3 => 0xc088_3c00, adma::k3::RegisterBlock;
    /// Real-time-domain half-duplex I2S0 peripheral.
    pub struct RI2S0 => 0xc088_3100, ri2s::k3::RegisterBlock;
    /// Real-time-domain half-duplex I2S1 peripheral.
    pub struct RI2S1 => 0xc088_3500, ri2s::k3::RegisterBlock;
    /// Real-time-domain half-duplex I2S2 peripheral.
    pub struct RI2S2 => 0xc088_3900, ri2s::k3::RegisterBlock;
    /// Real-time-domain half-duplex I2S3 peripheral.
    pub struct RI2S3 => 0xc088_3d00, ri2s::k3::RegisterBlock;
    /// eSPI host peripheral.
    pub struct ESPI => 0xcac8_c000, espi::k3::RegisterBlock;
    /// Infrared receiver 0.
    pub struct IR0 => 0xd401_7e00, ir::k3::RegisterBlock;
    /// Infrared receiver 1.
    pub struct IR1 => 0xd401_7f00, ir::k3::RegisterBlock;
    /// Real-time-domain infrared receiver 0.
    pub struct R_IR0 => 0xc088_7000, ir::k3::RegisterBlock;
    /// Real-time-domain infrared receiver 1.
    pub struct R_IR1 => 0xc088_e000, ir::k3::RegisterBlock;
    /// Non-secure mailbox 0.
    pub struct MAILBOX0 => 0xcac9_0000, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 1.
    pub struct MAILBOX1 => 0xcac9_0400, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 2.
    pub struct MAILBOX2 => 0xcac9_0800, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 3.
    pub struct MAILBOX3 => 0xcac9_0c00, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 4.
    pub struct MAILBOX4 => 0xcac9_1000, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 5.
    pub struct MAILBOX5 => 0xcac9_1400, mailbox::k3::RegisterBlock;
    /// Non-secure mailbox 6.
    pub struct MAILBOX6 => 0xcac9_1800, mailbox::k3::RegisterBlock;
    /// Hardware spinlock peripheral.
    pub struct SPINLOCK => 0xcac9_1c00, spinlock::k3::RegisterBlock;
    /// Temperature sensor peripheral.
    pub struct TSENSOR => 0xd401_8000, tsensor::k3::RegisterBlock;
    /// UFS host controller.
    pub struct UFS => 0xc0e0_0000, ufs::k3::RegisterBlock;
    /// UFS PHY-management registers.
    pub struct UFS_MNG => 0xc0e0_1b00, ufs::k3::ManagementRegisters;
    /// Secure configuration unit.
    pub struct SEC_CIU => 0xf058_0000, sec_ciu::k3::RegisterBlock;
    // IOPMP1..9 bases: K3 manual section 15.4.4.1, not enumerated by the live DT.
    // https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/15_security.md
    /// IOPMP1 configuration registers.
    pub struct IOPMP1 => 0xf080_0000, iopmp::k3::RegisterBlock;
    /// IOPMP2 configuration registers.
    pub struct IOPMP2 => 0xf085_0000, iopmp::k3::RegisterBlock;
    /// IOPMP3 configuration registers.
    pub struct IOPMP3 => 0xf087_0000, iopmp::k3::RegisterBlock;
    /// IOPMP4 configuration registers.
    pub struct IOPMP4 => 0xf086_0000, iopmp::k3::RegisterBlock;
    /// IOPMP5 configuration registers.
    pub struct IOPMP5 => 0xf088_0000, iopmp::k3::RegisterBlock;
    /// IOPMP6 configuration registers.
    pub struct IOPMP6 => 0xf081_0000, iopmp::k3::RegisterBlock;
    /// IOPMP7 configuration registers.
    pub struct IOPMP7 => 0xf082_0000, iopmp::k3::RegisterBlock;
    /// IOPMP8 configuration registers.
    pub struct IOPMP8 => 0xf083_0000, iopmp::k3::RegisterBlock;
    /// IOPMP9 configuration registers.
    pub struct IOPMP9 => 0xf084_0000, iopmp::k3::RegisterBlock;
    /// USB2 host PHY registers.
    pub struct USB2_PHY_HOST => 0xc0a2_0000, usb2_phy::k3::RegisterBlock;
    /// USB2 port A PHY registers.
    pub struct USB2_PHY_A => 0xcad2_0000, usb2_phy::k3::PortARegisterBlock;
    /// USB2 port B PHY registers.
    pub struct USB2_PHY_B => 0x8150_0000, usb2_phy::k3::RegisterBlock;
    /// USB2 port C PHY registers.
    pub struct USB2_PHY_C => 0x8180_0000, usb2_phy::k3::RegisterBlock;
    /// USB2 port D PHY registers.
    pub struct USB2_PHY_D => 0x81b0_0000, usb2_phy::k3::RegisterBlock;
    // PHY2/3/4 have both USB and PCIe DT nodes; each physical PHY has one owner.
    // https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/phy/spacemit/phy-k3-usb3.c
    /// HSIO PHY0 PCIe registers.
    pub struct HSIO_PHY0 => 0x81d0_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY1 PCIe registers.
    pub struct HSIO_PHY1 => 0x81e0_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY2 PCIe / USB3 combo registers.
    pub struct HSIO_PHY2 => 0x81f0_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY3 PCIe / USB3 combo registers.
    pub struct HSIO_PHY3 => 0x8200_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY4 PCIe / USB3 combo registers.
    pub struct HSIO_PHY4 => 0x8210_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY5 PCIe registers.
    pub struct HSIO_PHY5 => 0x8220_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY8 USB3 port A registers.
    pub struct HSIO_PHY8 => 0xcad3_0000, hsio_phy::k3::RegisterBlock;
    /// HSIO PHY9 USB3 port A registers.
    pub struct HSIO_PHY9 => 0xcad4_0000, hsio_phy::k3::RegisterBlock;
}

impl_clock_controller!(apbs, APBS, apbs::k3::RegisterBlock);
impl_clock_controller!(mpmu, MPMU, mpmu::k3::RegisterBlock);
impl_qspi!(QSPI);

impl_uart!(
    UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7, UART8, UART9, UART10, R_UART0, R_UART1,
    R_UART2, R_UART3, R_UART4, R_UART5
);

gpio_pads!(__new_k3);

// UART routes on GPIO0..127: K3 user manual, sections 3.4.4 through 3.4.8.
// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k3/k3_docs/k3_usermanual/03_pinout.md
// Dedicated pads (such as the PWR_SSP UART0 console route) are not GPIO<N>.
impl_uart_pads! {
    (42, 2): IntoTransmit, into_uart_transmit, UART0;
    (43, 2): IntoReceive, into_uart_receive, UART0;
    (93, 3): IntoTransmit, into_uart_transmit, UART0;
    (94, 3): IntoReceive, into_uart_receive, UART0;
    (97, 1): IntoTransmit, into_uart_transmit, UART2;
    (98, 1): IntoReceive, into_uart_receive, UART2;
    (26, 2): IntoTransmit, into_uart_transmit, UART3;
    (27, 2): IntoReceive, into_uart_receive, UART3;
    (55, 2): IntoReceive, into_uart_receive, UART3;
    (56, 2): IntoTransmit, into_uart_transmit, UART3;
    (86, 3): IntoTransmit, into_uart_transmit, UART4;
    (87, 3): IntoReceive, into_uart_receive, UART4;
    (99, 3): IntoTransmit, into_uart_transmit, UART4;
    (100, 3): IntoReceive, into_uart_receive, UART4;
    (21, 2): IntoTransmit, into_uart_transmit, UART5;
    (22, 2): IntoReceive, into_uart_receive, UART5;
    (82, 4): IntoReceive, into_uart_receive, UART5;
    (83, 4): IntoTransmit, into_uart_transmit, UART5;
    (48, 2): IntoTransmit, into_uart_transmit, UART6;
    (49, 2): IntoReceive, into_uart_receive, UART6;
    (122, 3): IntoTransmit, into_uart_transmit, UART6;
    (123, 3): IntoReceive, into_uart_receive, UART6;
    (13, 2): IntoTransmit, into_uart_transmit, UART7;
    (14, 2): IntoReceive, into_uart_receive, UART7;
    (23, 5): IntoTransmit, into_uart_transmit, UART7;
    (24, 5): IntoReceive, into_uart_receive, UART7;
    (11, 5): IntoReceive, into_uart_receive, UART8;
    (12, 5): IntoTransmit, into_uart_transmit, UART8;
    (76, 3): IntoTransmit, into_uart_transmit, UART8;
    (77, 3): IntoReceive, into_uart_receive, UART8;
    (84, 3): IntoTransmit, into_uart_transmit, UART9;
    (85, 3): IntoReceive, into_uart_receive, UART9;
    (31, 2): IntoTransmit, into_uart_transmit, UART10;
    (32, 2): IntoReceive, into_uart_receive, UART10;
    (44, 2): IntoTransmit, into_uart_transmit, UART10;
    (45, 2): IntoReceive, into_uart_receive, UART10;
}

apbc_clocks! {
    APBC, apbc::k3::RegisterBlock;
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
        UART10 => uart10, uart10_clock_reset;
    }
    i2c {
        I2C0 => i2c0, twsi0_clock_reset;
        I2C1 => i2c1, twsi1_clock_reset;
        I2C2 => i2c2, twsi2_clock_reset;
        I2C4 => i2c4, twsi4_clock_reset;
        I2C5 => i2c5, twsi5_clock_reset;
        I2C6 => i2c6, twsi6_clock_reset;
        I2C8 => i2c8, twsi8_clock_reset;
    }
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
    /// Hardware hart 8.
    pub hart8: crate::hart::Hart<8>,
    /// Hardware hart 9.
    pub hart9: crate::hart::Hart<9>,
    /// Hardware hart 10.
    pub hart10: crate::hart::Hart<10>,
    /// Hardware hart 11.
    pub hart11: crate::hart::Hart<11>,
    /// Hardware hart 12.
    pub hart12: crate::hart::Hart<12>,
    /// Hardware hart 13.
    pub hart13: crate::hart::Hart<13>,
    /// Hardware hart 14.
    pub hart14: crate::hart::Hart<14>,
    /// Hardware hart 15.
    pub hart15: crate::hart::Hart<15>,
}

/// K3 peripheral ownership.
pub struct Peripherals {
    /// Exclusive application-hart tokens.
    pub harts: Harts,
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
    /// I2C8 (TWSI8) peripheral.
    pub i2c8: I2C8,
    /// PLL clock-control peripheral.
    pub apbs: APBS,
    /// Main power-management peripheral.
    pub mpmu: MPMU,
    /// Quad-SPI memory-controller peripheral.
    pub qspi: QSPI,
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
    /// UART10 peripheral.
    pub uart10: UART10,
    /// Real-time-domain UART0 peripheral.
    pub r_uart0: R_UART0,
    /// Real-time-domain UART1 peripheral.
    pub r_uart1: R_UART1,
    /// Real-time-domain UART2 peripheral.
    pub r_uart2: R_UART2,
    /// Real-time-domain UART3 peripheral.
    pub r_uart3: R_UART3,
    /// Real-time-domain UART4 peripheral.
    pub r_uart4: R_UART4,
    /// Real-time-domain UART5 peripheral.
    pub r_uart5: R_UART5,
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
    /// Full-duplex I2S2 peripheral.
    pub i2s2: I2S2,
    /// Full-duplex I2S3 peripheral.
    pub i2s3: I2S3,
    /// Full-duplex I2S4 peripheral.
    pub i2s4: I2S4,
    /// Full-duplex I2S5 peripheral.
    pub i2s5: I2S5,
    /// SD / SDIO / eMMC host 0.
    pub sdh0: SDH0,
    /// SD / SDIO / eMMC host 1.
    pub sdh1: SDH1,
    /// SD / SDIO / eMMC host 2.
    pub sdh2: SDH2,
    /// Timer 0 and watchdog peripheral.
    pub timer0: TIMER0,
    /// Timer 1 and watchdog peripheral.
    pub timer1: TIMER1,
    /// Non-secure peripheral DMA controller.
    pub pdma: PDMA,
    /// Real-time-domain I2C0 peripheral.
    pub r_i2c0: R_I2C0,
    /// Real-time-domain I2C1 peripheral.
    pub r_i2c1: R_I2C1,
    /// Real-time-domain PWM0 legacy control/duty/period registers.
    pub r_pwm0: R_PWM0,
    /// Real-time-domain PWM1 legacy control/duty/period registers.
    pub r_pwm1: R_PWM1,
    /// Real-time-domain PWM2 legacy control/duty/period registers.
    pub r_pwm2: R_PWM2,
    /// Real-time-domain PWM3 legacy control/duty/period registers.
    pub r_pwm3: R_PWM3,
    /// Real-time-domain PWM4 legacy control/duty/period registers.
    pub r_pwm4: R_PWM4,
    /// Real-time-domain PWM5 legacy control/duty/period registers.
    pub r_pwm5: R_PWM5,
    /// Real-time-domain PWM6 legacy control/duty/period registers.
    pub r_pwm6: R_PWM6,
    /// Real-time-domain PWM7 legacy control/duty/period registers.
    pub r_pwm7: R_PWM7,
    /// Real-time-domain PWM8 legacy control/duty/period registers.
    pub r_pwm8: R_PWM8,
    /// Real-time-domain PWM9 legacy control/duty/period registers.
    pub r_pwm9: R_PWM9,
    /// SSP0 / SPI peripheral.
    pub spi0: SPI0,
    /// SSP1 / SPI peripheral.
    pub spi1: SPI1,
    /// Real-time-domain SSP0 / SPI peripheral.
    pub r_spi0: R_SPI0,
    /// Real-time-domain SSP1 / SPI peripheral.
    pub r_spi1: R_SPI1,
    /// Real-time-domain SSP2 / SPI peripheral.
    pub r_spi2: R_SPI2,
    /// PMU timer and watchdog peripheral.
    pub pmu_timer: PMU_TIMER,
    /// Secure timer 8 and watchdog peripheral.
    pub sec_timer: SEC_TIMER,
    /// HDMA0 (AIDMA0) controller.
    pub hdma0: HDMA0,
    /// HDMA1 (AIDMA1) controller.
    pub hdma1: HDMA1,
    /// HDMA2 (AIDMA2) controller.
    pub hdma2: HDMA2,
    /// HDMA3 (AIDMA3) controller.
    pub hdma3: HDMA3,
    /// HDMA4 (AIDMA4) controller.
    pub hdma4: HDMA4,
    /// HDMA5 (AIDMA5) controller.
    pub hdma5: HDMA5,
    /// HDMA6 (AIDMA6) controller.
    pub hdma6: HDMA6,
    /// HDMA7 (AIDMA7) controller.
    pub hdma7: HDMA7,
    /// CAN-FD0 peripheral.
    pub can0: CAN0,
    /// CAN-FD1 peripheral.
    pub can1: CAN1,
    /// CAN-FD2 peripheral.
    pub can2: CAN2,
    /// CAN-FD3 peripheral.
    pub can3: CAN3,
    /// CAN-FD4 peripheral.
    pub can4: CAN4,
    /// Real-time-domain CAN-FD0 peripheral.
    pub r_can0: R_CAN0,
    /// Real-time-domain CAN-FD1 peripheral.
    pub r_can1: R_CAN1,
    /// Real-time-domain CAN-FD2 peripheral.
    pub r_can2: R_CAN2,
    /// Real-time-domain CAN-FD3 peripheral.
    pub r_can3: R_CAN3,
    /// Real-time-domain CAN-FD4 peripheral.
    pub r_can4: R_CAN4,
    /// Audio DMA0 controller.
    pub adma0: ADMA0,
    /// Audio DMA1 controller.
    pub adma1: ADMA1,
    /// Audio DMA2 controller.
    pub adma2: ADMA2,
    /// Audio DMA3 controller.
    pub adma3: ADMA3,
    /// Real-time-domain half-duplex I2S0 peripheral.
    pub ri2s0: RI2S0,
    /// Real-time-domain half-duplex I2S1 peripheral.
    pub ri2s1: RI2S1,
    /// Real-time-domain half-duplex I2S2 peripheral.
    pub ri2s2: RI2S2,
    /// Real-time-domain half-duplex I2S3 peripheral.
    pub ri2s3: RI2S3,
    /// eSPI host peripheral.
    pub espi: ESPI,
    /// Infrared receiver 0.
    pub ir0: IR0,
    /// Infrared receiver 1.
    pub ir1: IR1,
    /// Real-time-domain infrared receiver 0.
    pub r_ir0: R_IR0,
    /// Real-time-domain infrared receiver 1.
    pub r_ir1: R_IR1,
    /// Non-secure mailbox 0.
    pub mailbox0: MAILBOX0,
    /// Non-secure mailbox 1.
    pub mailbox1: MAILBOX1,
    /// Non-secure mailbox 2.
    pub mailbox2: MAILBOX2,
    /// Non-secure mailbox 3.
    pub mailbox3: MAILBOX3,
    /// Non-secure mailbox 4.
    pub mailbox4: MAILBOX4,
    /// Non-secure mailbox 5.
    pub mailbox5: MAILBOX5,
    /// Non-secure mailbox 6.
    pub mailbox6: MAILBOX6,
    /// Hardware spinlock peripheral.
    pub spinlock: SPINLOCK,
    /// Temperature sensor peripheral.
    pub tsensor: TSENSOR,
    /// UFS host controller.
    pub ufs: UFS,
    /// UFS PHY-management registers.
    pub ufs_mng: UFS_MNG,
    /// Secure configuration unit.
    pub sec_ciu: SEC_CIU,
    /// IOPMP1 configuration registers.
    pub iopmp1: IOPMP1,
    /// IOPMP2 configuration registers.
    pub iopmp2: IOPMP2,
    /// IOPMP3 configuration registers.
    pub iopmp3: IOPMP3,
    /// IOPMP4 configuration registers.
    pub iopmp4: IOPMP4,
    /// IOPMP5 configuration registers.
    pub iopmp5: IOPMP5,
    /// IOPMP6 configuration registers.
    pub iopmp6: IOPMP6,
    /// IOPMP7 configuration registers.
    pub iopmp7: IOPMP7,
    /// IOPMP8 configuration registers.
    pub iopmp8: IOPMP8,
    /// IOPMP9 configuration registers.
    pub iopmp9: IOPMP9,
    /// USB2 host PHY registers.
    pub usb2_phy_host: USB2_PHY_HOST,
    /// USB2 port A PHY registers.
    pub usb2_phy_a: USB2_PHY_A,
    /// USB2 port B PHY registers.
    pub usb2_phy_b: USB2_PHY_B,
    /// USB2 port C PHY registers.
    pub usb2_phy_c: USB2_PHY_C,
    /// USB2 port D PHY registers.
    pub usb2_phy_d: USB2_PHY_D,
    /// HSIO PHY0 PCIe registers.
    pub hsio_phy0: HSIO_PHY0,
    /// HSIO PHY1 PCIe registers.
    pub hsio_phy1: HSIO_PHY1,
    /// HSIO PHY2 PCIe / USB3 combo registers.
    pub hsio_phy2: HSIO_PHY2,
    /// HSIO PHY3 PCIe / USB3 combo registers.
    pub hsio_phy3: HSIO_PHY3,
    /// HSIO PHY4 PCIe / USB3 combo registers.
    pub hsio_phy4: HSIO_PHY4,
    /// HSIO PHY5 PCIe registers.
    pub hsio_phy5: HSIO_PHY5,
    /// HSIO PHY8 USB3 port A registers.
    pub hsio_phy8: HSIO_PHY8,
    /// HSIO PHY9 USB3 port A registers.
    pub hsio_phy9: HSIO_PHY9,
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
    /// Run on K3 with aligned, identity-mapped registers accessible at the current
    /// privilege level, including secure and real-time peripherals.
    ///
    /// Retain valid power, upstream clocks and reset for every access,
    /// permanently for consumed tokens and pad/clock tokens.
    ///
    /// Stop conflicting users and DMA, including former pad users; no hart, firmware
    /// or duplicate owner may invalidate these guarantees, even after drop or forget.
    ///
    /// PLL sources must remain stable.
    ///
    /// Untracked clock consumers and DMA must tolerate shared-gate changes;
    /// HAL consumers must retain controller borrows.
    ///
    /// Used pads must have valid electrical settings.
    #[inline]
    pub unsafe fn steal() -> Self {
        super::PERIPHERALS_TAKEN.store(true, core::sync::atomic::Ordering::Release);
        Self {
            // SAFETY: These are all K3 application hart IDs; steal transfers them once.
            harts: Harts {
                hart0: unsafe { crate::hart::Hart::new() },
                hart1: unsafe { crate::hart::Hart::new() },
                hart2: unsafe { crate::hart::Hart::new() },
                hart3: unsafe { crate::hart::Hart::new() },
                hart4: unsafe { crate::hart::Hart::new() },
                hart5: unsafe { crate::hart::Hart::new() },
                hart6: unsafe { crate::hart::Hart::new() },
                hart7: unsafe { crate::hart::Hart::new() },
                hart8: unsafe { crate::hart::Hart::new() },
                hart9: unsafe { crate::hart::Hart::new() },
                hart10: unsafe { crate::hart::Hart::new() },
                hart11: unsafe { crate::hart::Hart::new() },
                hart12: unsafe { crate::hart::Hart::new() },
                hart13: unsafe { crate::hart::Hart::new() },
                hart14: unsafe { crate::hart::Hart::new() },
                hart15: unsafe { crate::hart::Hart::new() },
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
            uart10: UART10 {
                _private: core::marker::PhantomData,
            },
            r_uart0: R_UART0 {
                _private: core::marker::PhantomData,
            },
            r_uart1: R_UART1 {
                _private: core::marker::PhantomData,
            },
            r_uart2: R_UART2 {
                _private: core::marker::PhantomData,
            },
            r_uart3: R_UART3 {
                _private: core::marker::PhantomData,
            },
            r_uart4: R_UART4 {
                _private: core::marker::PhantomData,
            },
            r_uart5: R_UART5 {
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
            i2s2: I2S2 {
                _private: core::marker::PhantomData,
            },
            i2s3: I2S3 {
                _private: core::marker::PhantomData,
            },
            i2s4: I2S4 {
                _private: core::marker::PhantomData,
            },
            i2s5: I2S5 {
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
            timer0: TIMER0 {
                _private: core::marker::PhantomData,
            },
            timer1: TIMER1 {
                _private: core::marker::PhantomData,
            },
            pdma: PDMA {
                _private: core::marker::PhantomData,
            },
            r_i2c0: R_I2C0 {
                _private: core::marker::PhantomData,
            },
            r_i2c1: R_I2C1 {
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
            spi0: SPI0 {
                _private: core::marker::PhantomData,
            },
            spi1: SPI1 {
                _private: core::marker::PhantomData,
            },
            r_spi0: R_SPI0 {
                _private: core::marker::PhantomData,
            },
            r_spi1: R_SPI1 {
                _private: core::marker::PhantomData,
            },
            r_spi2: R_SPI2 {
                _private: core::marker::PhantomData,
            },
            pmu_timer: PMU_TIMER {
                _private: core::marker::PhantomData,
            },
            sec_timer: SEC_TIMER {
                _private: core::marker::PhantomData,
            },
            hdma0: HDMA0 {
                _private: core::marker::PhantomData,
            },
            hdma1: HDMA1 {
                _private: core::marker::PhantomData,
            },
            hdma2: HDMA2 {
                _private: core::marker::PhantomData,
            },
            hdma3: HDMA3 {
                _private: core::marker::PhantomData,
            },
            hdma4: HDMA4 {
                _private: core::marker::PhantomData,
            },
            hdma5: HDMA5 {
                _private: core::marker::PhantomData,
            },
            hdma6: HDMA6 {
                _private: core::marker::PhantomData,
            },
            hdma7: HDMA7 {
                _private: core::marker::PhantomData,
            },
            can0: CAN0 {
                _private: core::marker::PhantomData,
            },
            can1: CAN1 {
                _private: core::marker::PhantomData,
            },
            can2: CAN2 {
                _private: core::marker::PhantomData,
            },
            can3: CAN3 {
                _private: core::marker::PhantomData,
            },
            can4: CAN4 {
                _private: core::marker::PhantomData,
            },
            r_can0: R_CAN0 {
                _private: core::marker::PhantomData,
            },
            r_can1: R_CAN1 {
                _private: core::marker::PhantomData,
            },
            r_can2: R_CAN2 {
                _private: core::marker::PhantomData,
            },
            r_can3: R_CAN3 {
                _private: core::marker::PhantomData,
            },
            r_can4: R_CAN4 {
                _private: core::marker::PhantomData,
            },
            adma0: ADMA0 {
                _private: core::marker::PhantomData,
            },
            adma1: ADMA1 {
                _private: core::marker::PhantomData,
            },
            adma2: ADMA2 {
                _private: core::marker::PhantomData,
            },
            adma3: ADMA3 {
                _private: core::marker::PhantomData,
            },
            ri2s0: RI2S0 {
                _private: core::marker::PhantomData,
            },
            ri2s1: RI2S1 {
                _private: core::marker::PhantomData,
            },
            ri2s2: RI2S2 {
                _private: core::marker::PhantomData,
            },
            ri2s3: RI2S3 {
                _private: core::marker::PhantomData,
            },
            espi: ESPI {
                _private: core::marker::PhantomData,
            },
            ir0: IR0 {
                _private: core::marker::PhantomData,
            },
            ir1: IR1 {
                _private: core::marker::PhantomData,
            },
            r_ir0: R_IR0 {
                _private: core::marker::PhantomData,
            },
            r_ir1: R_IR1 {
                _private: core::marker::PhantomData,
            },
            mailbox0: MAILBOX0 {
                _private: core::marker::PhantomData,
            },
            mailbox1: MAILBOX1 {
                _private: core::marker::PhantomData,
            },
            mailbox2: MAILBOX2 {
                _private: core::marker::PhantomData,
            },
            mailbox3: MAILBOX3 {
                _private: core::marker::PhantomData,
            },
            mailbox4: MAILBOX4 {
                _private: core::marker::PhantomData,
            },
            mailbox5: MAILBOX5 {
                _private: core::marker::PhantomData,
            },
            mailbox6: MAILBOX6 {
                _private: core::marker::PhantomData,
            },
            spinlock: SPINLOCK {
                _private: core::marker::PhantomData,
            },
            tsensor: TSENSOR {
                _private: core::marker::PhantomData,
            },
            ufs: UFS {
                _private: core::marker::PhantomData,
            },
            ufs_mng: UFS_MNG {
                _private: core::marker::PhantomData,
            },
            sec_ciu: SEC_CIU {
                _private: core::marker::PhantomData,
            },
            iopmp1: IOPMP1 {
                _private: core::marker::PhantomData,
            },
            iopmp2: IOPMP2 {
                _private: core::marker::PhantomData,
            },
            iopmp3: IOPMP3 {
                _private: core::marker::PhantomData,
            },
            iopmp4: IOPMP4 {
                _private: core::marker::PhantomData,
            },
            iopmp5: IOPMP5 {
                _private: core::marker::PhantomData,
            },
            iopmp6: IOPMP6 {
                _private: core::marker::PhantomData,
            },
            iopmp7: IOPMP7 {
                _private: core::marker::PhantomData,
            },
            iopmp8: IOPMP8 {
                _private: core::marker::PhantomData,
            },
            iopmp9: IOPMP9 {
                _private: core::marker::PhantomData,
            },
            usb2_phy_host: USB2_PHY_HOST {
                _private: core::marker::PhantomData,
            },
            usb2_phy_a: USB2_PHY_A {
                _private: core::marker::PhantomData,
            },
            usb2_phy_b: USB2_PHY_B {
                _private: core::marker::PhantomData,
            },
            usb2_phy_c: USB2_PHY_C {
                _private: core::marker::PhantomData,
            },
            usb2_phy_d: USB2_PHY_D {
                _private: core::marker::PhantomData,
            },
            hsio_phy0: HSIO_PHY0 {
                _private: core::marker::PhantomData,
            },
            hsio_phy1: HSIO_PHY1 {
                _private: core::marker::PhantomData,
            },
            hsio_phy2: HSIO_PHY2 {
                _private: core::marker::PhantomData,
            },
            hsio_phy3: HSIO_PHY3 {
                _private: core::marker::PhantomData,
            },
            hsio_phy4: HSIO_PHY4 {
                _private: core::marker::PhantomData,
            },
            hsio_phy5: HSIO_PHY5 {
                _private: core::marker::PhantomData,
            },
            hsio_phy8: HSIO_PHY8 {
                _private: core::marker::PhantomData,
            },
            hsio_phy9: HSIO_PHY9 {
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
        register_type::<I2C8, i2c::RegisterBlock>();
        assert_eq!(I2C8::ptr() as usize, 0xd401_d800);
        register_type::<APBS, apbs::k3::RegisterBlock>();
        assert_eq!(APBS::ptr() as usize, 0xd409_0000);
        register_type::<MPMU, mpmu::k3::RegisterBlock>();
        fn controller_types<A, M>()
        where
            A: apbs::Instance<'static, RegisterBlock = apbs::k3::RegisterBlock>,
            M: mpmu::Instance<'static, RegisterBlock = mpmu::k3::RegisterBlock>,
            for<'a> &'a mut A: apbs::Instance<'a, RegisterBlock = apbs::k3::RegisterBlock>,
            for<'a> &'a mut M: mpmu::Instance<'a, RegisterBlock = mpmu::k3::RegisterBlock>,
        {
        }
        controller_types::<APBS, MPMU>();
        assert_eq!(MPMU::ptr() as usize, 0xd405_0000);
        register_type::<MFPR, mfpr::k3::RegisterBlock>();
        assert_eq!(MFPR::ptr() as usize, 0xd401_e000);
        register_type::<QSPI, qspi::RegisterBlock>();
        fn qspi_type<T: qspi::Instance<'static>>()
        where
            for<'a> &'a mut T: qspi::Instance<'a>,
        {
        }
        qspi_type::<QSPI>();
        assert_eq!(QSPI::ptr() as usize, 0xd420_c000);
        assert_eq!(
            APBS::ptr() as usize
                + core::mem::offset_of!(apbs::k3::RegisterBlock, pll1_software_control2),
            0xd409_0104
        );
        assert_eq!(
            MPMU::ptr() as usize
                + core::mem::offset_of!(mpmu::k3::RegisterBlock, application_clock_gate),
            0xd405_1024
        );
        fn apmu_type<
            T: Deref<Target = apmu::k3::RegisterBlock> + AsRef<apmu::k3::RegisterBlock>,
        >() {
        }
        fn gpio_type<
            T: Deref<Target = gpio::k3::RegisterBlock> + AsRef<gpio::k3::RegisterBlock>,
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
        uart_type::<UART10>();
        uart_type::<R_UART0>();
        uart_type::<R_UART1>();
        uart_type::<R_UART2>();
        uart_type::<R_UART3>();
        uart_type::<R_UART4>();
        uart_type::<R_UART5>();
        register_type::<APBC, apbc::k3::RegisterBlock>();
        assert_eq!(APBC::ptr() as usize, 0xd401_5000);
        assert_eq!(APMU::ptr() as usize, 0xd428_2800);
        register_type::<CIU, ciu::k3::RegisterBlock>();
        assert_eq!(CIU::ptr() as usize, 0xd428_2c00);
        assert_eq!(
            APMU::ptr() as usize + core::mem::offset_of!(apmu::k3::RegisterBlock, qspi_clock_reset),
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
        assert_eq!(UART10::ptr() as usize, 0xd401_f000);
        assert_eq!(R_UART0::ptr() as usize, 0xc088_1000);
        assert_eq!(R_UART1::ptr() as usize, 0xc088_1100);
        assert_eq!(R_UART2::ptr() as usize, 0xc088_1200);
        assert_eq!(R_UART3::ptr() as usize, 0xc088_1300);
        assert_eq!(R_UART4::ptr() as usize, 0xc088_1400);
        assert_eq!(R_UART5::ptr() as usize, 0xc088_1500);
        fn address<T: Deref<Target = R> + AsRef<R>, R>(ptr: *const R, expected: usize) {
            assert_eq!(ptr as usize, expected);
        }
        address::<I2C3, i2c::RegisterBlock>(I2C3::ptr(), 0xf061_4000);
        address::<PWM0, pwm::k3::RegisterBlock>(PWM0::ptr(), 0xd401_a000);
        address::<PWM1, pwm::k3::RegisterBlock>(PWM1::ptr(), 0xd401_a400);
        address::<PWM2, pwm::k3::RegisterBlock>(PWM2::ptr(), 0xd401_a800);
        address::<PWM3, pwm::k3::RegisterBlock>(PWM3::ptr(), 0xd401_ac00);
        address::<PWM4, pwm::k3::RegisterBlock>(PWM4::ptr(), 0xd401_b000);
        address::<PWM5, pwm::k3::RegisterBlock>(PWM5::ptr(), 0xd401_b400);
        address::<PWM6, pwm::k3::RegisterBlock>(PWM6::ptr(), 0xd401_b800);
        address::<PWM7, pwm::k3::RegisterBlock>(PWM7::ptr(), 0xd401_bc00);
        address::<PWM8, pwm::k3::RegisterBlock>(PWM8::ptr(), 0xd402_0000);
        address::<PWM9, pwm::k3::RegisterBlock>(PWM9::ptr(), 0xd402_0400);
        address::<PWM10, pwm::k3::RegisterBlock>(PWM10::ptr(), 0xd402_0800);
        address::<PWM11, pwm::k3::RegisterBlock>(PWM11::ptr(), 0xd402_0c00);
        address::<PWM12, pwm::k3::RegisterBlock>(PWM12::ptr(), 0xd402_1000);
        address::<PWM13, pwm::k3::RegisterBlock>(PWM13::ptr(), 0xd402_1400);
        address::<PWM14, pwm::k3::RegisterBlock>(PWM14::ptr(), 0xd402_1800);
        address::<PWM15, pwm::k3::RegisterBlock>(PWM15::ptr(), 0xd402_1c00);
        address::<PWM16, pwm::k3::RegisterBlock>(PWM16::ptr(), 0xd402_2000);
        address::<PWM17, pwm::k3::RegisterBlock>(PWM17::ptr(), 0xd402_2400);
        address::<PWM18, pwm::k3::RegisterBlock>(PWM18::ptr(), 0xd402_2800);
        address::<PWM19, pwm::k3::RegisterBlock>(PWM19::ptr(), 0xd402_2c00);
        address::<SPI2, spi::k3::RegisterBlock>(SPI2::ptr(), 0xf061_3000);
        address::<SPI3, spi::k3::RegisterBlock>(SPI3::ptr(), 0xd401_c000);
        address::<I2S0, spi::k3::RegisterBlock>(I2S0::ptr(), 0xd402_6000);
        address::<I2S1, spi::k3::RegisterBlock>(I2S1::ptr(), 0xd402_6800);
        address::<I2S2, spi::k3::RegisterBlock>(I2S2::ptr(), 0xd402_7000);
        address::<I2S3, spi::k3::RegisterBlock>(I2S3::ptr(), 0xd402_7800);
        address::<I2S4, spi::k3::RegisterBlock>(I2S4::ptr(), 0xd404_1000);
        address::<I2S5, spi::k3::RegisterBlock>(I2S5::ptr(), 0xd404_1800);
        address::<SDH0, sdh::k3::RegisterBlock>(SDH0::ptr(), 0xd428_0000);
        address::<SDH1, sdh::k3::RegisterBlock>(SDH1::ptr(), 0xd428_0800);
        address::<SDH2, sdh::k3::RegisterBlock>(SDH2::ptr(), 0xd428_1000);
        address::<TIMER0, timer::k3::RegisterBlock>(TIMER0::ptr(), 0xd401_4000);
        address::<TIMER1, timer::k3::RegisterBlock>(TIMER1::ptr(), 0xd401_6000);
        address::<PDMA, pdma::k3::RegisterBlock>(PDMA::ptr(), 0xd400_0000);
        address::<R_I2C0, i2c::RegisterBlock>(R_I2C0::ptr(), 0xc088_6000);
        address::<R_I2C1, i2c::RegisterBlock>(R_I2C1::ptr(), 0xc088_6100);
        address::<R_PWM0, pwm::k1::RegisterBlock>(R_PWM0::ptr(), 0xc088_d100);
        address::<R_PWM1, pwm::k1::RegisterBlock>(R_PWM1::ptr(), 0xc088_d200);
        address::<R_PWM2, pwm::k1::RegisterBlock>(R_PWM2::ptr(), 0xc088_d300);
        address::<R_PWM3, pwm::k1::RegisterBlock>(R_PWM3::ptr(), 0xc088_d400);
        address::<R_PWM4, pwm::k1::RegisterBlock>(R_PWM4::ptr(), 0xc088_d500);
        address::<R_PWM5, pwm::k1::RegisterBlock>(R_PWM5::ptr(), 0xc088_d600);
        address::<R_PWM6, pwm::k1::RegisterBlock>(R_PWM6::ptr(), 0xc088_d700);
        address::<R_PWM7, pwm::k1::RegisterBlock>(R_PWM7::ptr(), 0xc088_d800);
        address::<R_PWM8, pwm::k1::RegisterBlock>(R_PWM8::ptr(), 0xc088_d900);
        address::<R_PWM9, pwm::k1::RegisterBlock>(R_PWM9::ptr(), 0xc088_da00);
        address::<SPI0, spi::k3::RegisterBlock>(SPI0::ptr(), 0xd404_0000);
        address::<SPI1, spi::k3::RegisterBlock>(SPI1::ptr(), 0xd404_0800);
        address::<R_SPI0, spi::k3::RegisterBlock>(R_SPI0::ptr(), 0xc088_5000);
        address::<R_SPI1, spi::k3::RegisterBlock>(R_SPI1::ptr(), 0xc088_5100);
        address::<R_SPI2, spi::k3::RegisterBlock>(R_SPI2::ptr(), 0xc088_5200);
        address::<PMU_TIMER, timer::k3::RegisterBlock>(PMU_TIMER::ptr(), 0xd408_0000);
        address::<SEC_TIMER, timer::k3::RegisterBlock>(SEC_TIMER::ptr(), 0xf061_6000);
        address::<HDMA0, hdma::k3::RegisterBlock>(HDMA0::ptr(), 0xd880_4000);
        address::<HDMA1, hdma::k3::RegisterBlock>(HDMA1::ptr(), 0xd880_5000);
        address::<HDMA2, hdma::k3::RegisterBlock>(HDMA2::ptr(), 0xd880_6000);
        address::<HDMA3, hdma::k3::RegisterBlock>(HDMA3::ptr(), 0xd880_7000);
        address::<HDMA4, hdma::k3::RegisterBlock>(HDMA4::ptr(), 0xd880_8000);
        address::<HDMA5, hdma::k3::RegisterBlock>(HDMA5::ptr(), 0xd880_9000);
        address::<HDMA6, hdma::k3::RegisterBlock>(HDMA6::ptr(), 0xd880_a000);
        address::<HDMA7, hdma::k3::RegisterBlock>(HDMA7::ptr(), 0xd880_b000);
        address::<CAN0, can::k3::RegisterBlock>(CAN0::ptr(), 0xd402_8000);
        address::<CAN1, can::k3::RegisterBlock>(CAN1::ptr(), 0xd402_c000);
        address::<CAN2, can::k3::RegisterBlock>(CAN2::ptr(), 0xd403_4000);
        address::<CAN3, can::k3::RegisterBlock>(CAN3::ptr(), 0xd403_8000);
        address::<CAN4, can::k3::RegisterBlock>(CAN4::ptr(), 0xd403_c000);
        address::<R_CAN0, can::k3::RegisterBlock>(R_CAN0::ptr(), 0xc071_0000);
        address::<R_CAN1, can::k3::RegisterBlock>(R_CAN1::ptr(), 0xc072_0000);
        address::<R_CAN2, can::k3::RegisterBlock>(R_CAN2::ptr(), 0xc073_0000);
        address::<R_CAN3, can::k3::RegisterBlock>(R_CAN3::ptr(), 0xc074_0000);
        address::<R_CAN4, can::k3::RegisterBlock>(R_CAN4::ptr(), 0xc075_0000);
        address::<ADMA0, adma::k3::RegisterBlock>(ADMA0::ptr(), 0xc088_3000);
        address::<ADMA1, adma::k3::RegisterBlock>(ADMA1::ptr(), 0xc088_3400);
        address::<ADMA2, adma::k3::RegisterBlock>(ADMA2::ptr(), 0xc088_3800);
        address::<ADMA3, adma::k3::RegisterBlock>(ADMA3::ptr(), 0xc088_3c00);
        address::<RI2S0, ri2s::k3::RegisterBlock>(RI2S0::ptr(), 0xc088_3100);
        address::<RI2S1, ri2s::k3::RegisterBlock>(RI2S1::ptr(), 0xc088_3500);
        address::<RI2S2, ri2s::k3::RegisterBlock>(RI2S2::ptr(), 0xc088_3900);
        address::<RI2S3, ri2s::k3::RegisterBlock>(RI2S3::ptr(), 0xc088_3d00);
        address::<ESPI, espi::k3::RegisterBlock>(ESPI::ptr(), 0xcac8_c000);
        address::<IR0, ir::k3::RegisterBlock>(IR0::ptr(), 0xd401_7e00);
        address::<IR1, ir::k3::RegisterBlock>(IR1::ptr(), 0xd401_7f00);
        address::<R_IR0, ir::k3::RegisterBlock>(R_IR0::ptr(), 0xc088_7000);
        address::<R_IR1, ir::k3::RegisterBlock>(R_IR1::ptr(), 0xc088_e000);
        address::<MAILBOX0, mailbox::k3::RegisterBlock>(MAILBOX0::ptr(), 0xcac9_0000);
        address::<MAILBOX1, mailbox::k3::RegisterBlock>(MAILBOX1::ptr(), 0xcac9_0400);
        address::<MAILBOX2, mailbox::k3::RegisterBlock>(MAILBOX2::ptr(), 0xcac9_0800);
        address::<MAILBOX3, mailbox::k3::RegisterBlock>(MAILBOX3::ptr(), 0xcac9_0c00);
        address::<MAILBOX4, mailbox::k3::RegisterBlock>(MAILBOX4::ptr(), 0xcac9_1000);
        address::<MAILBOX5, mailbox::k3::RegisterBlock>(MAILBOX5::ptr(), 0xcac9_1400);
        address::<MAILBOX6, mailbox::k3::RegisterBlock>(MAILBOX6::ptr(), 0xcac9_1800);
        address::<SPINLOCK, spinlock::k3::RegisterBlock>(SPINLOCK::ptr(), 0xcac9_1c00);
        address::<TSENSOR, tsensor::k3::RegisterBlock>(TSENSOR::ptr(), 0xd401_8000);
        address::<UFS, ufs::k3::RegisterBlock>(UFS::ptr(), 0xc0e0_0000);
        address::<UFS_MNG, ufs::k3::ManagementRegisters>(UFS_MNG::ptr(), 0xc0e0_1b00);
        address::<SEC_CIU, sec_ciu::k3::RegisterBlock>(SEC_CIU::ptr(), 0xf058_0000);
        address::<IOPMP1, iopmp::k3::RegisterBlock>(IOPMP1::ptr(), 0xf080_0000);
        address::<IOPMP2, iopmp::k3::RegisterBlock>(IOPMP2::ptr(), 0xf085_0000);
        address::<IOPMP3, iopmp::k3::RegisterBlock>(IOPMP3::ptr(), 0xf087_0000);
        address::<IOPMP4, iopmp::k3::RegisterBlock>(IOPMP4::ptr(), 0xf086_0000);
        address::<IOPMP5, iopmp::k3::RegisterBlock>(IOPMP5::ptr(), 0xf088_0000);
        address::<IOPMP6, iopmp::k3::RegisterBlock>(IOPMP6::ptr(), 0xf081_0000);
        address::<IOPMP7, iopmp::k3::RegisterBlock>(IOPMP7::ptr(), 0xf082_0000);
        address::<IOPMP8, iopmp::k3::RegisterBlock>(IOPMP8::ptr(), 0xf083_0000);
        address::<IOPMP9, iopmp::k3::RegisterBlock>(IOPMP9::ptr(), 0xf084_0000);
        address::<USB2_PHY_HOST, usb2_phy::k3::RegisterBlock>(USB2_PHY_HOST::ptr(), 0xc0a2_0000);
        address::<USB2_PHY_A, usb2_phy::k3::PortARegisterBlock>(USB2_PHY_A::ptr(), 0xcad2_0000);
        address::<USB2_PHY_B, usb2_phy::k3::RegisterBlock>(USB2_PHY_B::ptr(), 0x8150_0000);
        address::<USB2_PHY_C, usb2_phy::k3::RegisterBlock>(USB2_PHY_C::ptr(), 0x8180_0000);
        address::<USB2_PHY_D, usb2_phy::k3::RegisterBlock>(USB2_PHY_D::ptr(), 0x81b0_0000);
        address::<HSIO_PHY0, hsio_phy::k3::RegisterBlock>(HSIO_PHY0::ptr(), 0x81d0_0000);
        address::<HSIO_PHY1, hsio_phy::k3::RegisterBlock>(HSIO_PHY1::ptr(), 0x81e0_0000);
        address::<HSIO_PHY2, hsio_phy::k3::RegisterBlock>(HSIO_PHY2::ptr(), 0x81f0_0000);
        address::<HSIO_PHY3, hsio_phy::k3::RegisterBlock>(HSIO_PHY3::ptr(), 0x8200_0000);
        address::<HSIO_PHY4, hsio_phy::k3::RegisterBlock>(HSIO_PHY4::ptr(), 0x8210_0000);
        address::<HSIO_PHY5, hsio_phy::k3::RegisterBlock>(HSIO_PHY5::ptr(), 0x8220_0000);
        address::<HSIO_PHY8, hsio_phy::k3::RegisterBlock>(HSIO_PHY8::ptr(), 0xcad3_0000);
        address::<HSIO_PHY9, hsio_phy::k3::RegisterBlock>(HSIO_PHY9::ptr(), 0xcad4_0000);
        assert_eq!(core::mem::size_of::<Peripherals>(), 0);
    }
}
