#![no_std]
#![no_main]

use rot_bootloader::{Board, eeprom, entry, eprintln, println};

static EEPROM_BYTES: spin::Mutex<[u8; 256]> = spin::Mutex::new([0; 256]);

#[entry]
fn main(mut b: Board) {
    println!("Hello world!");
    let mut bytes = EEPROM_BYTES.lock();
    if let Err(error) = b.eeprom.read_data(0, &mut *bytes) {
        eprintln!("EEPROM read failed: {:?}", error);
        return;
    }
    let mut info = eeprom::EepromInfo::default();
    if let Err(error) = eeprom::parse_into(&*bytes, &mut info) {
        eprintln!("EEPROM parse failed: {:?}", error);
        return;
    }

    println!("EEPROM:");
    rot_bootloader::io::print_eeprom(&info);

    println!("DDR: LPDDR4X, 2 CS, 2400 MT/s; training...");
    // SAFETY: Cold MUSE Card M1 boot; no payload, DMA or secondary hart uses DRAM.
    let firmware_status = match unsafe { b.init_ddr(info.ddr.as_ref()) } {
        Ok(status) => status,
        Err(error) => {
            eprintln!("DDR initialization failed: {:?}", error);
            return;
        }
    };
    println!("DDR firmware returned: {}", firmware_status);
    println!("NOR: loading SBI firmware, next stage and DTB...");
    // SAFETY: Training succeeded; DRAM is unused and all other harts and DMA remain stopped.
    match unsafe { b.load_images() } {
        Ok(images) => {
            println!(
                "Loaded: SBI {} bytes, next stage {} bytes, DTB {} bytes",
                images.sbi.size, images.payload.size, images.dtb.size
            );
            // SAFETY: Verified images, sole boot hart, completed PIO and no further Rust execution.
            unsafe { rot_bootloader::handoff::enter(images) };
        }
        Err(error) => eprintln!("NOR image loading failed: {:?}", error),
    }
}
