//! K3 peripheral ownership and addresses.

use spacemit_hal::{apbc, apbs, apmu, ciu, gpio, i2c, mfpr, mpmu, qspi, uart};

// Address map: Linux k3.dtsi and the K3 pico-ITX running device tree.
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// /proc/device-tree/aliases/serial1 selects /soc/serial@f0612000.
// R_UART0..5 name the real-time-domain nodes (spacemit,rcpu-uart), whose
// serial11..16 aliases select /soc/serial@c0881000 through serial@c0881500.

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
    /// privilege level, including secure UART1.
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
        assert_eq!(core::mem::size_of::<Peripherals>(), 0);
    }
}
