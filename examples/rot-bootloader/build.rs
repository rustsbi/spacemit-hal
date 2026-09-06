use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=link.x");
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "none" {
        return;
    }
    assert_eq!(env::var("TARGET").unwrap(), "riscv64imac-unknown-none-elf");
    let script = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("link.x");
    println!(
        "cargo:rustc-link-arg-bin=rot-bootloader=-T{}",
        script.display()
    );
    println!("cargo:rustc-link-arg-bin=rot-bootloader=--no-relax");
}
