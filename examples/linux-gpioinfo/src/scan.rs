//! Scans the running device tree for SpacemiT GPIO controllers.
//!
//! K3 DTS fragment; syscon_apbc, saplic, and pinctrl come from the SoC DTS.
//! The parent uses two address cells, two size cells, and identity translation.
//! The 0x800-byte region covers all four banks, including GPIO3 at offset 0x100.
//! References:
//! <https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi>
//! <https://github.com/torvalds/linux/blob/master/Documentation/devicetree/bindings/gpio/spacemit,k1-gpio.yaml>
//!
//! ```dts
//! #include <dt-bindings/clock/spacemit,k3-clocks.h>
//! #include <dt-bindings/interrupt-controller/irq.h>
//!
//! / {
//!     soc {
//!         #address-cells = <2>;
//!         #size-cells = <2>;
//!         ranges;
//!
//!         gpio: gpio@d4019000 {
//!             compatible = "spacemit,k3-gpio";
//!             reg = <0x0 0xd4019000 0x0 0x800>;
//!             clocks = <&syscon_apbc CLK_APBC_GPIO>,
//!                      <&syscon_apbc CLK_APBC_GPIO_BUS>;
//!             clock-names = "core", "bus";
//!             gpio-controller;
//!             #gpio-cells = <3>;
//!             interrupt-parent = <&saplic>;
//!             interrupts = <58 IRQ_TYPE_LEVEL_HIGH>;
//!             interrupt-controller;
//!             #interrupt-cells = <3>;
//!             gpio-ranges = <&pinctrl 0 0 0 32>,
//!                           <&pinctrl 1 0 32 32>,
//!                           <&pinctrl 2 0 64 32>,
//!                           <&pinctrl 3 0 96 32>;
//!         };
//!     };
//! };
//! ```

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Soc {
    K1,
    K3,
}

pub(crate) struct Gpio {
    pub(crate) soc: Soc,
    pub(crate) node: PathBuf,
    pub(crate) address: u64,
}

fn compatible_soc(compatible: &[u8]) -> Option<Soc> {
    compatible
        .split(|&byte| byte == 0)
        .find_map(|name| match name {
            b"spacemit,k1-gpio" => Some(Soc::K1),
            b"spacemit,k3-gpio" => Some(Soc::K3),
            _ => None,
        })
}

fn reg_address(reg: &[u8], address_cells: &[u8]) -> io::Result<u64> {
    match address_cells {
        [0, 0, 0, 1] if reg.len() >= 4 => {
            Ok(u32::from_be_bytes(reg[..4].try_into().unwrap()).into())
        }
        [0, 0, 0, 2] if reg.len() >= 8 => Ok(u64::from_be_bytes(reg[..8].try_into().unwrap())),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid GPIO reg or #address-cells",
        )),
    }
}

pub(crate) fn scan(soc_path: &Path) -> io::Result<Vec<Gpio>> {
    // Empty ranges means the SoC bus addresses are physical addresses.
    if !fs::read(soc_path.join("ranges"))?.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "non-identity soc ranges",
        ));
    }
    let address_cells = fs::read(soc_path.join("#address-cells"))?;
    let mut gpios = Vec::new();
    for entry in fs::read_dir(soc_path)? {
        let entry = entry?;
        if !entry.file_name().as_encoded_bytes().starts_with(b"gpio@") {
            continue;
        }
        let node = entry.path();
        let compatible = match fs::read(node.join("compatible")) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        };
        if let Some(soc) = compatible_soc(&compatible) {
            let address = reg_address(&fs::read(node.join("reg"))?, &address_cells)?;
            gpios.push(Gpio { soc, node, address });
        }
    }
    if gpios.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no spacemit,k1-gpio or spacemit,k3-gpio in /soc",
        ));
    }
    gpios.sort_by(|a, b| a.node.cmp(&b.node));
    Ok(gpios)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_complete_compatible_strings() {
        assert_eq!(compatible_soc(b"spacemit,k1-gpio\0"), Some(Soc::K1));
        assert_eq!(compatible_soc(b"syscon\0spacemit,k3-gpio\0"), Some(Soc::K3));
        assert_eq!(compatible_soc(b"spacemit,k3-gpio-extra\0"), None);
        assert_eq!(compatible_soc(b"spacemit,k3-pico-itx\0"), None);
    }

    #[test]
    fn decodes_reg_base_instead_of_assuming_an_address() {
        assert_eq!(
            reg_address(&0x1234_5000u32.to_be_bytes(), &[0, 0, 0, 1]).unwrap(),
            0x1234_5000
        );
        let reg = [0u32, 0xd401_9000, 0, 0x100].map(u32::to_be_bytes).concat();
        assert_eq!(reg_address(&reg, &[0, 0, 0, 2]).unwrap(), 0xd401_9000);
        assert!(reg_address(&reg[..7], &[0, 0, 0, 2]).is_err());
        assert!(reg_address(&reg, &[0, 0, 0, 3]).is_err());
    }
}
