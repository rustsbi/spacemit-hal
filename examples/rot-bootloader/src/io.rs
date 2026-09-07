//! Locked standard output.

use crate::console::{self, Guard};
use embedded_io::{Error as _, ErrorKind, ErrorType, Write};
use spacemit_hal::uart::BlockingUart;

/// A console lock released on drop; unavailable handles report WriteZero.
#[must_use]
pub struct Stdout {
    guard: Option<Guard<'static, BlockingUart<'static>>>,
}

/// Tries to lock the initializing hart's UART until the handle is dropped.
pub fn stdout() -> Stdout {
    Stdout {
        guard: console::lock(),
    }
}

impl Stdout {
    fn uart(&mut self) -> Result<&mut BlockingUart<'static>, ErrorKind> {
        self.guard.as_deref_mut().ok_or(ErrorKind::WriteZero)
    }
}

impl ErrorType for Stdout {
    type Error = ErrorKind;
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, ErrorKind> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.uart()?.write(buf).map_err(|error| error.kind())
    }

    fn flush(&mut self) -> Result<(), ErrorKind> {
        self.uart()?.flush();
        Ok(())
    }
}

/// An optional byte slice displayed as a hexadecimal list, or `None`, by `ufmt`.
pub struct Hex<'a>(pub Option<&'a [u8]>);

impl ufmt::uDisplay for Hex<'_> {
    fn fmt<W: ufmt::uWrite + ?Sized>(
        &self,
        f: &mut ufmt::Formatter<'_, W>,
    ) -> Result<(), W::Error> {
        let Some(bytes) = self.0 else {
            return f.write_str("None");
        };
        f.write_str("[")?;
        let mut index = 0;
        while index < bytes.len() {
            if index != 0 {
                f.write_str(", ")?;
            }
            const DIGITS: &[u8; 16] = b"0123456789abcdef";
            let byte = bytes[index];
            let pair = [DIGITS[(byte >> 4) as usize], DIGITS[(byte & 15) as usize]];
            // SAFETY: Both bytes come from the ASCII-only digit table.
            f.write_str(unsafe { core::str::from_utf8_unchecked(&pair) })?;
            index += 1;
        }
        f.write_str("]")
    }
}

/// Prints EEPROM fields using shared formatting code for each field type.
#[inline(never)]
pub fn print_eeprom(info: &crate::eeprom::EepromInfo<'_>) {
    text_field("product_name", info.product_name);
    text_field("part_number", info.part_number);
    text_field("serial_number", info.serial_number);
    mac_field("base_mac", info.base_mac.as_ref());
    text_field("manufacture_date", info.manufacture_date);
    debug_field("device_version", info.device_version);
    text_field("label_revision", info.label_revision);
    text_field("platform_name", info.platform_name);
    text_field("onie_version", info.onie_version);
    debug_field("mac_count", info.mac_count);
    text_field("manufacturer", info.manufacturer);
    text_field("country_code", info.country_code);
    text_field("vendor", info.vendor);
    text_field("diagnostic_version", info.diagnostic_version);
    text_field("service_tag", info.service_tag);
    debug_field("sdk_version", info.sdk_version);
    if let Some(ddr) = &info.ddr {
        debug_field("ddr chip_selects", ddr.chip_selects);
        text_field("ddr memory_type", ddr.memory_type);
        debug_field("ddr data_rate", ddr.data_rate);
        debug_field("ddr tx_odt", ddr.tx_odt);
    } else {
        crate::println!("ddr: None");
    }
    mac_field("wifi_mac", info.wifi_mac.as_ref());
    mac_field("bluetooth_address", info.bluetooth_address.as_ref());
    text_field("pmic_type", info.pmic_type);
    debug_field("eeprom_i2c_index", info.eeprom_i2c_index);
    debug_field("eeprom_pin_group", info.eeprom_pin_group);
}

#[inline(never)]
fn text_field(name: &str, value: Option<&str>) {
    crate::println!("{}: {}", name, value.unwrap_or("<unset>"));
}

#[inline(never)]
fn debug_field<T: ufmt::uDebug>(name: &str, value: T) {
    crate::println!("{}: {:?}", name, value);
}

#[inline(never)]
fn mac_field(name: &str, value: Option<&[u8; 6]>) {
    crate::println!("{}: {}", name, Hex(value.map(|mac| mac.as_slice())));
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[derive(Default)]
    struct Buffer(std::string::String);
    impl ufmt::uWrite for Buffer {
        type Error = core::convert::Infallible;
        fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
            self.0.push_str(text);
            Ok(())
        }
    }

    #[test]
    fn hex_preserves_absence_empty_slices_and_byte_padding() {
        let mut out = Buffer::default();
        ufmt::uwrite!(
            &mut out,
            "{} {} {}",
            Hex(None),
            Hex(Some(&[])),
            Hex(Some(&[0, 1, 0xab, 0xff]))
        )
        .unwrap();
        assert_eq!(out.0, "None [] [00, 01, ab, ff]");
        for byte in 0..=u8::MAX {
            let mut out = Buffer::default();
            ufmt::uwrite!(&mut out, "{}", Hex(Some(&[byte]))).unwrap();
            assert_eq!(out.0, std::format!("[{byte:02x}]"));
        }
    }
}
