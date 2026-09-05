//! K3 peripheral ownership and addresses.

use spacemit_hal::{apbc, apbs, apmu, gpio, i2c, mfpr, mpmu, qspi, uart};

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

/// K3 peripheral ownership.
pub struct Peripherals {
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
    /// Multi-function pad peripheral.
    pub mfpr: MFPR,
    /// Quad-SPI memory-controller peripheral.
    pub qspi: QSPI,
    /// APB clock and reset peripheral.
    pub apbc: APBC,
    /// Application-processor power, clock, and reset peripheral.
    pub apmu: APMU,
    /// GPIO peripheral.
    pub gpio: GPIO,
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
    /// Acquires peripheral tokens without initializing hardware or checking ownership.
    ///
    /// # Safety
    ///
    /// The caller must run on K3 with each used register block identity-mapped,
    /// aligned, and accessible at the current privilege level, including UART1's
    /// secure domain; power, clocks, and reset must permit every register access.
    /// These conditions must hold for all register borrows, and no other tokens,
    /// drivers, harts, interrupt handlers, DMA, or OS may concurrently access the
    /// same peripherals, including through the other SoC module.
    #[inline]
    pub const unsafe fn steal() -> Self {
        Self {
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
            mfpr: MFPR {
                _private: core::marker::PhantomData,
            },
            qspi: QSPI {
                _private: core::marker::PhantomData,
            },
            apbc: APBC {
                _private: core::marker::PhantomData,
            },
            apmu: APMU {
                _private: core::marker::PhantomData,
            },
            gpio: GPIO {
                _private: core::marker::PhantomData,
            },
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
        assert_eq!(MPMU::ptr() as usize, 0xd405_0000);
        register_type::<MFPR, mfpr::k3::RegisterBlock>();
        assert_eq!(MFPR::ptr() as usize, 0xd401_e000);
        register_type::<QSPI, qspi::RegisterBlock>();
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
        fn apbc_type<
            T: Deref<Target = apbc::k3::RegisterBlock> + AsRef<apbc::k3::RegisterBlock>,
        >() {
        }
        fn apmu_type<
            T: Deref<Target = apmu::k3::RegisterBlock> + AsRef<apmu::k3::RegisterBlock>,
        >() {
        }
        fn gpio_type<
            T: Deref<Target = gpio::k3::RegisterBlock> + AsRef<gpio::k3::RegisterBlock>,
        >() {
        }
        fn uart_type<T: Deref<Target = uart::RegisterBlock> + AsRef<uart::RegisterBlock>>() {}

        apbc_type::<APBC>();
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
        assert_eq!(APBC::ptr() as usize, 0xd401_5000);
        assert_eq!(APMU::ptr() as usize, 0xd428_2800);
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
