//! SpacemiT P1 (SPM8821) PMIC driver.
#![no_std]

use embedded_hal::i2c::I2c;
use measurements::Voltage;

// Vendor U-Boot: include/power/spacemit/spm8821.h.
const ADDRESS: u8 = 0x41;

/// A PMIC operation failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error<E> {
    /// An I²C transaction failed.
    I2c(E),
    /// A register write failed readback verification.
    Readback,
    /// The voltage is outside the supported range or steps.
    Voltage,
}

/// A buck converter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Buck {
    Buck1,
    Buck2,
    Buck3,
    Buck4,
    Buck5,
    Buck6,
}

/// An SPM8821 PMIC owning its I²C interface.
pub struct Spm8821<I> {
    i2c: I,
}

impl<I: I2c> Spm8821<I> {
    /// Wraps the board's P1 bus without accessing registers.
    #[inline]
    pub const fn new(i2c: I) -> Self {
        Self { i2c }
    }

    /// Sets an exactly representable buck voltage.
    #[inline]
    pub fn set_buck_voltage(
        &mut self,
        buck: Buck,
        voltage: Voltage,
    ) -> Result<(), Error<I::Error>> {
        let selector = voltage_selector(voltage).ok_or(Error::Voltage)?;
        self.update(0x48 + 3 * buck as u8, 0xff, selector)
    }

    /// Enables or disables a buck converter without changing its voltage.
    #[inline]
    pub fn set_buck_enabled(&mut self, buck: Buck, enabled: bool) -> Result<(), Error<I::Error>> {
        self.update(0x47 + 3 * buck as u8, 1, u8::from(enabled))
    }

    /// Enables a buck converter without changing its voltage.
    #[inline]
    pub fn enable_buck(&mut self, buck: Buck) -> Result<(), Error<I::Error>> {
        self.set_buck_enabled(buck, true)
    }

    /// Disables a buck converter without changing its voltage.
    #[inline]
    pub fn disable_buck(&mut self, buck: Buck) -> Result<(), Error<I::Error>> {
        self.set_buck_enabled(buck, false)
    }

    #[inline]
    fn read(&mut self, register: u8) -> Result<u8, Error<I::Error>> {
        let mut value = [0];
        self.i2c
            .write_read(ADDRESS, &[register], &mut value)
            .map_err(Error::I2c)?;
        Ok(value[0])
    }

    #[inline]
    fn update(&mut self, register: u8, mask: u8, bits: u8) -> Result<(), Error<I::Error>> {
        let expected = (self.read(register)? & !mask) | (bits & mask);
        self.i2c
            .write(ADDRESS, &[register, expected])
            .map_err(Error::I2c)?;
        if self.read(register)? != expected {
            return Err(Error::Readback);
        }
        Ok(())
    }
}

#[inline]
fn voltage_selector(voltage: Voltage) -> Option<u8> {
    let value = voltage.as_microvolts();
    if !value.is_finite() {
        return None;
    }
    let microvolts = (value + 0.5) as u32;
    // Allow conversion roundoff, not fractional-microvolt requests.
    if (value - f64::from(microvolts)).abs() > value.abs() * 4.0 * f64::EPSILON {
        return None;
    }
    let (base, step, first) = match microvolts {
        500_000..=1_350_000 => (500_000, 5_000, 0),
        1_375_000..=3_450_000 => (1_375_000, 25_000, 0xab),
        _ => return None,
    };
    let offset = microvolts - base;
    offset
        .is_multiple_of(step)
        .then_some((offset / step + first) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::Infallible;
    use embedded_hal::i2c::{ErrorType, Operation};

    struct Bus {
        registers: [u8; 256],
        ignore_writes: bool,
    }

    impl ErrorType for Bus {
        type Error = Infallible;
    }

    impl I2c for Bus {
        fn transaction(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Infallible> {
            assert_eq!(address, 0x41);
            match operations {
                [Operation::Write([register]), Operation::Read(value)] => {
                    assert_eq!(value.len(), 1);
                    value[0] = self.registers[*register as usize];
                }
                [Operation::Write([register, value])] => {
                    if !self.ignore_writes {
                        self.registers[*register as usize] = *value;
                    }
                }
                _ => panic!("unexpected transaction"),
            }
            Ok(())
        }
    }

    #[test]
    fn voltage_steps() {
        for (voltage, selector) in [
            (500_000, 0),
            (1_050_000, 0x6e),
            (1_350_000, 0xaa),
            (1_375_000, 0xab),
            (3_450_000, 0xfe),
        ] {
            assert_eq!(
                voltage_selector(Voltage::from_microvolts(f64::from(voltage))),
                Some(selector)
            );
        }
        for voltage in [
            f64::NEG_INFINITY,
            -1.0,
            0.0,
            500_000.1,
            500_001.0,
            1_350_001.0,
            1_374_999.0,
            3_450_001.0,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
        ] {
            assert_eq!(voltage_selector(Voltage::from_microvolts(voltage)), None);
        }
        for selector in 0..=0xfe_u8 {
            let microvolts = if selector <= 0xaa {
                500_000 + u32::from(selector) * 5_000
            } else {
                1_375_000 + u32::from(selector - 0xab) * 25_000
            };
            let voltage = Voltage::from_millivolts(f64::from(microvolts) / 1000.0);
            assert_eq!(voltage_selector(voltage), Some(selector));
        }
    }

    #[test]
    fn buck_configuration() {
        let mut bus = Bus {
            registers: [0; 256],
            ignore_writes: false,
        };
        bus.registers[0x47] = 0xa0;
        let mut pmic = Spm8821::new(&mut bus);
        assert_eq!(
            pmic.set_buck_voltage(Buck::Buck1, Voltage::from_microvolts(-1.0)),
            Err(Error::Voltage)
        );
        assert_eq!(pmic.i2c.registers[0x48], 0);
        pmic.set_buck_voltage(Buck::Buck1, Voltage::from_millivolts(1050.0))
            .unwrap();
        pmic.enable_buck(Buck::Buck1).unwrap();
        pmic.enable_buck(Buck::Buck3).unwrap();
        assert_eq!(pmic.i2c.registers[0x48], 0x6e);
        assert_eq!(pmic.i2c.registers[0x47], 0xa1);
        assert_eq!(pmic.i2c.registers[0x4d], 1);
        pmic.disable_buck(Buck::Buck1).unwrap();
        assert_eq!(pmic.i2c.registers[0x47], 0xa0);
        assert_eq!(pmic.i2c.registers[0x48], 0x6e);
        pmic.enable_buck(Buck::Buck1).unwrap();
        pmic.i2c.ignore_writes = true;
        assert_eq!(pmic.disable_buck(Buck::Buck1), Err(Error::Readback));
    }
}
