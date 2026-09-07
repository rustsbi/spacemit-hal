use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=link.x");
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "none" {
        return;
    }
    assert_eq!(env::var("TARGET").unwrap(), "riscv64imac-unknown-none-elf");
    if env::var_os("CARGO_FEATURE_DDR").is_some() {
        let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("../../vendor/spacemit-firmware/k1/v0.2/ddr_fw.bin");
        println!("cargo:rerun-if-changed={}", path.display());
        let firmware = fs::read(&path).unwrap_or_else(|error| {
            panic!(
                "read {}: {error}; run `git submodule update --init --recursive`",
                path.display()
            )
        });
        assert_eq!(
            format!("{:x}", Sha256::digest(&firmware)),
            "4ca729dae39580fe6d52ff6a1ead95fd18cd6394c942cc4a323882ccf2840148",
            "expected the unmodified K1 v0.2 DDR firmware"
        );
    }
    let script = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("link.x");
    println!("cargo:rustc-link-arg-bins=-T{}", script.display());
    println!("cargo:rustc-link-arg-bins=--no-relax");
}
