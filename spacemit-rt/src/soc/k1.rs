//! K1/M1 peripheral ownership and addresses.

use spacemit_hal::{apbc, apbs, apmu, ciu, counter, gpio, i2c, mfpr, mpmu, qspi, uart};

// Address map: Linux k1.dtsi and the vendor k1-x.dtsi (UART1 and R_UART0/1).
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// https://gitee.com/spacemit-buildroot/linux-6.6/blob/k1-bl-v2.2.y/arch/riscv/boot/dts/spacemit/k1-x.dtsi
// UART1 is in the secure domain; R_UART1 is at 0xc088_d000, unlike K3.

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
    /// privilege level, including secure UART1.
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
        assert_eq!(core::mem::size_of::<Peripherals>(), 0);
    }
}
