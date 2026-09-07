#![no_std]
#![no_main]

use rot_bootloader::{Board, eeprom, entry, println};

#[entry]
fn main(mut b: Board) {
    println!("Hello world!");
    let mut bytes = [0; 256];
    if let Err(error) = b.eeprom.read_data(0, &mut bytes) {
        println!("EEPROM read failed: {error:?}");
        return;
    }
    match &eeprom::parse(&bytes) {
        Ok(info) => {
            // Print fields separately to limit boot-stack usage.
            println!("EEPROM:");
            println!("product_name: {:?}", info.product_name);
            println!("part_number: {:?}", info.part_number);
            println!("serial_number: {:?}", info.serial_number);
            println!("base_mac: {:02x?}", info.base_mac);
            println!("manufacture_date: {:?}", info.manufacture_date);
            println!("device_version: {:?}", info.device_version);
            println!("label_revision: {:?}", info.label_revision);
            println!("platform_name: {:?}", info.platform_name);
            println!("onie_version: {:?}", info.onie_version);
            println!("mac_count: {:?}", info.mac_count);
            println!("manufacturer: {:?}", info.manufacturer);
            println!("country_code: {:?}", info.country_code);
            println!("vendor: {:?}", info.vendor);
            println!("diagnostic_version: {:?}", info.diagnostic_version);
            println!("service_tag: {:?}", info.service_tag);
            println!("sdk_version: {:?}", info.sdk_version);
            println!("ddr: {:?}", info.ddr);
            println!("wifi_mac: {:02x?}", info.wifi_mac);
            println!("bluetooth_address: {:02x?}", info.bluetooth_address);
            println!("pmic_type: {:?}", info.pmic_type);
            println!("eeprom_i2c_index: {:?}", info.eeprom_i2c_index);
            println!("eeprom_pin_group: {:?}", info.eeprom_pin_group);
        }
        Err(error) => println!("EEPROM parse failed: {error:?}"),
    }
}
