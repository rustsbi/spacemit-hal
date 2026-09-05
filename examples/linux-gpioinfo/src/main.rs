//! Prints the four SpacemiT GPIO direction registers on Linux.

mod mmio;
mod scan;

use spacemit_hal::gpio::{k1, k3};
use std::{env, io, path::Path};

enum RegisterBlock<'a> {
    K1(&'a k1::RegisterBlock),
    K3(&'a k3::RegisterBlock),
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "read".into());
    if !matches!(command.as_str(), "scan" | "read") || args.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: linux-gpioinfo [scan|read]",
        ));
    }
    for gpio in scan::scan(Path::new("/proc/device-tree/soc"))? {
        let description = format!(
            "{:?}: {} reg base = {:#x}",
            gpio.soc,
            gpio.node.display(),
            gpio.address
        );
        if command == "scan" {
            println!("{description}");
            continue;
        }
        eprintln!("{description}");
        // SAFETY: Trust the running DT to identify the GPIO block and its base.
        // The Linux driver must keep it powered until this mapping is dropped.
        let mapping = unsafe { mmio::Mapping::new(gpio.soc, gpio.address)? };
        let directions = match mapping.registers() {
            RegisterBlock::K1(gpio) => [
                gpio.pin_direction[0].read(),
                gpio.pin_direction[1].read(),
                gpio.pin_direction[2].read(),
                gpio.gpio3.pin_direction.read(),
            ],
            RegisterBlock::K3(gpio) => [
                gpio.gpio0.pin_direction.read(),
                gpio.gpio1.pin_direction.read(),
                gpio.gpio2.pin_direction.read(),
                gpio.gpio3.pin_direction.read(),
            ],
        };
        for (bank, direction) in directions.into_iter().enumerate() {
            println!("gpio{bank}: {direction:#010x}");
        }
    }
    Ok(())
}
