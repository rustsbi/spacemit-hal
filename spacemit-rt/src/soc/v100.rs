//! V100 peripheral ownership and physical addresses.
//!
//! Addresses come from the V100 EDK2 ACPI DSDT/SSDT resources and MADT,
//! collected on 2026-09-23 (firmware 1.0.0, 2026-06-05).
//! These devices use ACPI IDs, not device-tree compatible strings.
//! I²C numbering follows ACPI instances, not Linux adapter numbering.
//! Only confirmed fixed MMIO devices are exposed; PCI BARs are assigned at runtime.
//! Startup, clocks and pin multiplexing are not initialized by this module.

use spacemit_hal::{aplic, i2c, iommu, ipmi, uart};

soc! {
    /// 16550A UART (RSCV0003), byte-wide registers.
    pub struct UART0 => 0x53fe003f8, uart::v100::RegisterBlock;
    /// DesignWare I²C 0 (SET2000:00, CID INT33C2).
    pub struct I2C0 => 0x552200000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 1 (SET2000:01, CID INT33C2).
    pub struct I2C1 => 0x552210000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 2 (SET2000:02, CID INT33C2).
    pub struct I2C2 => 0x552220000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 3 (SET2000:03, CID INT33C2).
    pub struct I2C3 => 0x552230000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 4 (SET2000:04, CID INT33C2).
    pub struct I2C4 => 0x552240000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 5 (SET2000:05, CID INT33C2).
    pub struct I2C5 => 0x552250000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 6 (SET2000:06, CID INT33C2).
    pub struct I2C6 => 0x752200000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 7 (SET2000:07, CID INT33C2).
    pub struct I2C7 => 0x752210000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 8 (SET2000:08, CID INT33C2).
    pub struct I2C8 => 0x752220000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 9 (SET2000:09, CID INT33C2).
    pub struct I2C9 => 0x752230000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 10 (SET2000:0a, CID INT33C2).
    pub struct I2C10 => 0x752240000, i2c::v100::RegisterBlock;
    /// DesignWare I²C 11 (SET2000:0b, CID INT33C2).
    pub struct I2C11 => 0x752250000, i2c::v100::RegisterBlock;
    /// Supervisor APLIC 0 (RSCV0002), 511 interrupt sources.
    pub struct SAPLIC0 => 0x558500000, aplic::v100::RegisterBlock;
    /// Supervisor APLIC 1 (RSCV0002), 511 interrupt sources.
    pub struct SAPLIC1 => 0x758500000, aplic::v100::RegisterBlock;
    /// RISC-V IOMMU 0 (RSCV0004).
    pub struct IOMMU0 => 0x4780000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 1 (RSCV0004).
    pub struct IOMMU1 => 0x4f80000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 2 (RSCV0004).
    pub struct IOMMU2 => 0x5780000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 3 (RSCV0004).
    pub struct IOMMU3 => 0x5f80000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 4 (RSCV0004).
    pub struct IOMMU4 => 0x6780000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 5 (RSCV0004).
    pub struct IOMMU5 => 0x6f80000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 6 (RSCV0004).
    pub struct IOMMU6 => 0x7780000000, iommu::RegisterBlock;
    /// RISC-V IOMMU 7 (RSCV0004).
    pub struct IOMMU7 => 0x7f80000000, iommu::RegisterBlock;
    /// IPMI KCS host interface (IPI0001), byte-wide registers.
    pub struct IPMI_KCS => 0x53fe00ca2, ipmi::RegisterBlock;
}

/// Exclusive V100 peripheral tokens.
pub struct Peripherals {
    /// 16550A UART (RSCV0003), byte-wide registers.
    pub uart0: UART0,
    /// DesignWare I²C 0 (SET2000:00, CID INT33C2).
    pub i2c0: I2C0,
    /// DesignWare I²C 1 (SET2000:01, CID INT33C2).
    pub i2c1: I2C1,
    /// DesignWare I²C 2 (SET2000:02, CID INT33C2).
    pub i2c2: I2C2,
    /// DesignWare I²C 3 (SET2000:03, CID INT33C2).
    pub i2c3: I2C3,
    /// DesignWare I²C 4 (SET2000:04, CID INT33C2).
    pub i2c4: I2C4,
    /// DesignWare I²C 5 (SET2000:05, CID INT33C2).
    pub i2c5: I2C5,
    /// DesignWare I²C 6 (SET2000:06, CID INT33C2).
    pub i2c6: I2C6,
    /// DesignWare I²C 7 (SET2000:07, CID INT33C2).
    pub i2c7: I2C7,
    /// DesignWare I²C 8 (SET2000:08, CID INT33C2).
    pub i2c8: I2C8,
    /// DesignWare I²C 9 (SET2000:09, CID INT33C2).
    pub i2c9: I2C9,
    /// DesignWare I²C 10 (SET2000:0a, CID INT33C2).
    pub i2c10: I2C10,
    /// DesignWare I²C 11 (SET2000:0b, CID INT33C2).
    pub i2c11: I2C11,
    /// Supervisor APLIC 0 (RSCV0002), 511 interrupt sources.
    pub saplic0: SAPLIC0,
    /// Supervisor APLIC 1 (RSCV0002), 511 interrupt sources.
    pub saplic1: SAPLIC1,
    /// RISC-V IOMMU 0 (RSCV0004).
    pub iommu0: IOMMU0,
    /// RISC-V IOMMU 1 (RSCV0004).
    pub iommu1: IOMMU1,
    /// RISC-V IOMMU 2 (RSCV0004).
    pub iommu2: IOMMU2,
    /// RISC-V IOMMU 3 (RSCV0004).
    pub iommu3: IOMMU3,
    /// RISC-V IOMMU 4 (RSCV0004).
    pub iommu4: IOMMU4,
    /// RISC-V IOMMU 5 (RSCV0004).
    pub iommu5: IOMMU5,
    /// RISC-V IOMMU 6 (RSCV0004).
    pub iommu6: IOMMU6,
    /// RISC-V IOMMU 7 (RSCV0004).
    pub iommu7: IOMMU7,
    /// IPMI KCS host interface (IPI0001), byte-wide registers.
    pub ipmi_kcs: IPMI_KCS,
}

impl Peripherals {
    /// Acquires peripheral tokens once across all harts and SoC modules.
    ///
    /// # Safety
    /// The hardware-access requirements of [`Self::steal`] must hold.
    pub unsafe fn take() -> Option<Self> {
        if !super::claim_peripherals(&super::PERIPHERALS_TAKEN) {
            return None;
        }
        // SAFETY: The caller establishes hardware access; the atomic claims ownership.
        Some(unsafe { Self::steal() })
    }

    /// Acquires tokens without initializing hardware or checking ownership.
    ///
    /// # Safety
    /// Run on the matching V100 platform with aligned, identity-mapped registers
    /// accessible at the current privilege level. Keep mappings, power, clocks
    /// and reset valid for every access, permanently for consumed tokens.
    /// Stop conflicting users and DMA; no hart, firmware or duplicate owner may
    /// invalidate these guarantees, even after dropping or forgetting tokens.
    pub unsafe fn steal() -> Self {
        super::PERIPHERALS_TAKEN.store(true, core::sync::atomic::Ordering::Release);
        Self {
            uart0: UART0 {
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
            i2c3: I2C3 {
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
            i2c9: I2C9 {
                _private: core::marker::PhantomData,
            },
            i2c10: I2C10 {
                _private: core::marker::PhantomData,
            },
            i2c11: I2C11 {
                _private: core::marker::PhantomData,
            },
            saplic0: SAPLIC0 {
                _private: core::marker::PhantomData,
            },
            saplic1: SAPLIC1 {
                _private: core::marker::PhantomData,
            },
            iommu0: IOMMU0 {
                _private: core::marker::PhantomData,
            },
            iommu1: IOMMU1 {
                _private: core::marker::PhantomData,
            },
            iommu2: IOMMU2 {
                _private: core::marker::PhantomData,
            },
            iommu3: IOMMU3 {
                _private: core::marker::PhantomData,
            },
            iommu4: IOMMU4 {
                _private: core::marker::PhantomData,
            },
            iommu5: IOMMU5 {
                _private: core::marker::PhantomData,
            },
            iommu6: IOMMU6 {
                _private: core::marker::PhantomData,
            },
            iommu7: IOMMU7 {
                _private: core::marker::PhantomData,
            },
            ipmi_kcs: IPMI_KCS {
                _private: core::marker::PhantomData,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_addresses() {
        assert_eq!(UART0::BASE, 0x53fe003f8);
        if usize::BITS >= 64 {
            assert_eq!(UART0::ptr() as u64, UART0::BASE);
        }
        assert_eq!(I2C0::BASE, 0x552200000);
        if usize::BITS >= 64 {
            assert_eq!(I2C0::ptr() as u64, I2C0::BASE);
        }
        assert_eq!(I2C1::BASE, 0x552210000);
        if usize::BITS >= 64 {
            assert_eq!(I2C1::ptr() as u64, I2C1::BASE);
        }
        assert_eq!(I2C2::BASE, 0x552220000);
        if usize::BITS >= 64 {
            assert_eq!(I2C2::ptr() as u64, I2C2::BASE);
        }
        assert_eq!(I2C3::BASE, 0x552230000);
        if usize::BITS >= 64 {
            assert_eq!(I2C3::ptr() as u64, I2C3::BASE);
        }
        assert_eq!(I2C4::BASE, 0x552240000);
        if usize::BITS >= 64 {
            assert_eq!(I2C4::ptr() as u64, I2C4::BASE);
        }
        assert_eq!(I2C5::BASE, 0x552250000);
        if usize::BITS >= 64 {
            assert_eq!(I2C5::ptr() as u64, I2C5::BASE);
        }
        assert_eq!(I2C6::BASE, 0x752200000);
        if usize::BITS >= 64 {
            assert_eq!(I2C6::ptr() as u64, I2C6::BASE);
        }
        assert_eq!(I2C7::BASE, 0x752210000);
        if usize::BITS >= 64 {
            assert_eq!(I2C7::ptr() as u64, I2C7::BASE);
        }
        assert_eq!(I2C8::BASE, 0x752220000);
        if usize::BITS >= 64 {
            assert_eq!(I2C8::ptr() as u64, I2C8::BASE);
        }
        assert_eq!(I2C9::BASE, 0x752230000);
        if usize::BITS >= 64 {
            assert_eq!(I2C9::ptr() as u64, I2C9::BASE);
        }
        assert_eq!(I2C10::BASE, 0x752240000);
        if usize::BITS >= 64 {
            assert_eq!(I2C10::ptr() as u64, I2C10::BASE);
        }
        assert_eq!(I2C11::BASE, 0x752250000);
        if usize::BITS >= 64 {
            assert_eq!(I2C11::ptr() as u64, I2C11::BASE);
        }
        assert_eq!(SAPLIC0::BASE, 0x558500000);
        if usize::BITS >= 64 {
            assert_eq!(SAPLIC0::ptr() as u64, SAPLIC0::BASE);
        }
        assert_eq!(SAPLIC1::BASE, 0x758500000);
        if usize::BITS >= 64 {
            assert_eq!(SAPLIC1::ptr() as u64, SAPLIC1::BASE);
        }
        assert_eq!(IOMMU0::BASE, 0x4780000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU0::ptr() as u64, IOMMU0::BASE);
        }
        assert_eq!(IOMMU1::BASE, 0x4f80000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU1::ptr() as u64, IOMMU1::BASE);
        }
        assert_eq!(IOMMU2::BASE, 0x5780000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU2::ptr() as u64, IOMMU2::BASE);
        }
        assert_eq!(IOMMU3::BASE, 0x5f80000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU3::ptr() as u64, IOMMU3::BASE);
        }
        assert_eq!(IOMMU4::BASE, 0x6780000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU4::ptr() as u64, IOMMU4::BASE);
        }
        assert_eq!(IOMMU5::BASE, 0x6f80000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU5::ptr() as u64, IOMMU5::BASE);
        }
        assert_eq!(IOMMU6::BASE, 0x7780000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU6::ptr() as u64, IOMMU6::BASE);
        }
        assert_eq!(IOMMU7::BASE, 0x7f80000000);
        if usize::BITS >= 64 {
            assert_eq!(IOMMU7::ptr() as u64, IOMMU7::BASE);
        }
        assert_eq!(IPMI_KCS::BASE, 0x53fe00ca2);
        if usize::BITS >= 64 {
            assert_eq!(IPMI_KCS::ptr() as u64, IPMI_KCS::BASE);
        }
        assert_eq!(core::mem::size_of::<Peripherals>(), 0);
    }
}
