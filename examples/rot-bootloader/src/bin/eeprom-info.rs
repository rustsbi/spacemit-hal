#![no_std]
#![no_main]

use rot_bootloader::{Board, eeprom, entry, eprintln, io::Hex, println};

static EEPROM_BYTES: spin::Mutex<[u8; 256]> = spin::Mutex::new([0; 256]);

#[entry]
fn main(mut b: Board) {
    println!("SpacemiT EEPROM diagnostics.");
    let mut bytes = EEPROM_BYTES.lock();
    if let Err(error) = b.eeprom.read_data(0, &mut *bytes) {
        eprintln!("EEPROM read failed: {:?}", error);
        return;
    }
    println!("EEPROM raw:");
    for (index, chunk) in bytes.chunks(16).enumerate() {
        let offset = index * 16;
        println!("{:02x}: {}", offset, Hex(Some(chunk)));
    }
    let mut info = eeprom::EepromInfo::default();
    match eeprom::parse_into(&*bytes, &mut info) {
        Ok(()) => {
            println!("EEPROM parsed:");
            rot_bootloader::io::print_eeprom(&info);
        }
        Err(error) => eprintln!("EEPROM parse failed: {:?}", error),
    }
}
