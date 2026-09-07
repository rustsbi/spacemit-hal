//! Reads SpacemiT's ONIE-compatible board EEPROM.

use crate::{Error, Result};

/// DDR fields recorded in EEPROM, without board defaults.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DdrConfig<'a> {
    /// Populated chip selects.
    pub chip_selects: Option<u8>,
    /// Memory type.
    pub memory_type: Option<&'a str>,
    /// Transfer rate in MT/s.
    pub data_rate: Option<u16>,
    /// Transmit termination resistance in ohms.
    pub tx_odt: Option<u8>,
}

/// Board information borrowed from a validated EEPROM image.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EepromInfo<'a> {
    /// Product name.
    pub product_name: Option<&'a str>,
    /// Board part number.
    pub part_number: Option<&'a str>,
    /// Board serial number.
    pub serial_number: Option<&'a str>,
    /// Base Ethernet MAC address.
    pub base_mac: Option<[u8; 6]>,
    /// Manufacture date as MM/DD/YYYY HH:MM:SS.
    pub manufacture_date: Option<&'a str>,
    /// Hardware revision.
    pub device_version: Option<u8>,
    /// Label revision.
    pub label_revision: Option<&'a str>,
    /// Platform name.
    pub platform_name: Option<&'a str>,
    /// Factory ONIE version.
    pub onie_version: Option<&'a str>,
    /// Number of sequential Ethernet MAC addresses.
    pub mac_count: Option<u16>,
    /// Manufacturer name.
    pub manufacturer: Option<&'a str>,
    /// Two-letter manufacturing country code.
    pub country_code: Option<&'a str>,
    /// Vendor name.
    pub vendor: Option<&'a str>,
    /// Diagnostic software version.
    pub diagnostic_version: Option<&'a str>,
    /// Vendor service tag.
    pub service_tag: Option<&'a str>,
    /// SpacemiT SDK version number.
    pub sdk_version: Option<u32>,
    /// DDR information, absent when no DDR TLVs exist.
    pub ddr: Option<DdrConfig<'a>>,
    /// Wi-Fi MAC address.
    pub wifi_mac: Option<[u8; 6]>,
    /// Bluetooth address.
    pub bluetooth_address: Option<[u8; 6]>,
    /// PMIC model name.
    pub pmic_type: Option<&'a str>,
    /// EEPROM I²C controller index.
    pub eeprom_i2c_index: Option<u8>,
    /// EEPROM I²C pin group.
    pub eeprom_pin_group: Option<u8>,
}

/// Validates the TLV image and reads its optional board fields.
pub fn parse(bytes: &[u8]) -> Result<EepromInfo<'_>> {
    if bytes.len() < 17 || &bytes[..8] != b"TlvInfo\0" || bytes[8] != 1 {
        return Err(Error::Format);
    }
    let end = 11 + u16::from_be_bytes([bytes[9], bytes[10]]) as usize;
    if !(17..=2048).contains(&end) || end > bytes.len() || bytes[end - 6..end - 4] != [0xfe, 4] {
        return Err(Error::Format);
    }
    if crc32(&bytes[..end - 4]) != u32::from_be_bytes(bytes[end - 4..end].try_into().unwrap()) {
        return Err(Error::Checksum);
    }
    let mut info = EepromInfo::default();
    let mut fields = &bytes[11..end - 6];
    while !fields.is_empty() {
        let header = fields.get(..2).ok_or(Error::Format)?;
        let next = 2 + usize::from(header[1]);
        let value = fields.get(2..next).ok_or(Error::Format)?;
        // ONIE type codes and vendor include/configs/k1-x.h.
        match header[0] {
            0x21 => set(&mut info.product_name, text(value)?)?,
            0x22 => set(&mut info.part_number, text(value)?)?,
            0x23 => set(&mut info.serial_number, text(value)?)?,
            0x24 => set(&mut info.base_mac, *array(value)?)?,
            0x25 => set(&mut info.manufacture_date, text(array::<19>(value)?)?)?,
            0x26 => set(&mut info.device_version, array::<1>(value)?[0])?,
            0x27 => set(&mut info.label_revision, text(value)?)?,
            0x28 => set(&mut info.platform_name, text(value)?)?,
            0x29 => set(&mut info.onie_version, text(value)?)?,
            0x2a => set(&mut info.mac_count, u16::from_be_bytes(*array(value)?))?,
            0x2b => set(&mut info.manufacturer, text(value)?)?,
            0x2c => set(&mut info.country_code, text(array::<2>(value)?)?)?,
            0x2d => set(&mut info.vendor, text(value)?)?,
            0x2e => set(&mut info.diagnostic_version, text(value)?)?,
            0x2f => set(&mut info.service_tag, text(value)?)?,
            0x40 => set(&mut info.sdk_version, number(value)?)?,
            code @ 0x41..=0x44 => {
                let ddr = info.ddr.get_or_insert_with(DdrConfig::default);
                match code {
                    0x41 => set(&mut ddr.chip_selects, array::<1>(value)?[0])?,
                    0x42 => set(&mut ddr.memory_type, text(value)?)?,
                    0x43 => set(&mut ddr.data_rate, u16::from_be_bytes(*array(value)?))?,
                    _ => set(&mut ddr.tx_odt, array::<1>(value)?[0])?,
                }
            }
            0x60 => set(&mut info.wifi_mac, *array(value)?)?,
            0x61 => set(&mut info.bluetooth_address, *array(value)?)?,
            0x80 => set(&mut info.pmic_type, text(value)?)?,
            0x81 => set(&mut info.eeprom_i2c_index, array::<1>(value)?[0])?,
            0x82 => set(&mut info.eeprom_pin_group, array::<1>(value)?[0])?,
            0x00 | 0xfe | 0xff => return Err(Error::Format),
            _ => {}
        }
        fields = &fields[next..];
    }
    Ok(info)
}

fn set<T>(field: &mut Option<T>, value: T) -> Result<()> {
    if field.replace(value).is_some() {
        return Err(Error::Format);
    }
    Ok(())
}

fn array<const N: usize>(bytes: &[u8]) -> Result<&[u8; N]> {
    bytes.try_into().map_err(|_| Error::Format)
}

fn text(bytes: &[u8]) -> Result<&str> {
    let bytes = bytes.strip_suffix(&[0]).unwrap_or(bytes);
    if !bytes.is_ascii() || bytes.contains(&0) {
        return Err(Error::Format);
    }
    core::str::from_utf8(bytes).map_err(|_| Error::Format)
}

fn number(bytes: &[u8]) -> Result<u32> {
    if !(1..=4).contains(&bytes.len()) {
        return Err(Error::Format);
    }
    Ok(bytes
        .iter()
        .fold(0, |value, &byte| (value << 8) | u32::from(byte)))
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}
