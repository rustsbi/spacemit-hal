# rot-bootloader

The board entry reuses `spacemit-rt` startup and initializes MUSE Card M1 or MUSE Pi Pro devices:

```rust
use rot_bootloader::{Board, entry, println};

#[entry]
fn main(b: Board) {
    println!("PLL1: {} Hz", b.clocks.pll1().0);
}
```

`Board` exposes `eeprom`, `pmic`, and `clocks`. UART0 belongs exclusively to the
global console; other raw peripherals stay inside the entry's static storage.
EEPROM reading and DDR configuration parsing belong to `main`, not the entry.
Initialization failure returns to the runtime without calling `main`.

`println!` writes and flushes on the initializing hart. Calls before console
initialization, on other harts, or during reentrant output are skipped.
`print!` / `println!` use `ufmt`: pass arguments explicitly (`"{}", value`, not
`"{value}"`). Only types with native or derived `uDisplay` / `uDebug` support work.
`io::Hex` displays optional byte slices in hexadecimal. `eprintln!` is a separate,
cold `core::fmt` path for external errors and dynamic panic messages.

Install Rust target:

```
rustup target add riscv64imac-unknown-none-elf
```

Initialize the pinned [SpacemiT firmware](https://github.com/spacemit-com/spacemit-firmware) submodule at `vendor/spacemit-firmware` and run from the workspace root:

```powershell
git submodule update --init --recursive
cargo run -p rot-bootloader --release --target riscv64imac-unknown-none-elf
```

Connect in BootROM download mode; the runner downloads to RAM, never writes flash.
UART0 uses 115200 8N1. Both boards share UART, EEPROM, PMIC, DDR and QSPI setup in
`platform/mod.rs`; `main` passes the EEPROM product name to `load_images` to select
the vendor FIT configuration, or its default when the product name is absent.
NOR partition offsets come from its CRC-checked environment.

Retain `DDR-FIRMWARE-LICENSE` when distributing the resulting image.
The default binary trains DDR and destructively checks unused DRAM; `hello-world`
and `eeprom-info` do not call DDR initialization.

## SRAM layout

The [vendor K1 configuration](https://github.com/spacemit-com/uboot-2022.10/blob/1fa1ca64e9705a3650bcc7c21f6666949290830f/configs/k1_defconfig)
sets the SPL code-region ceiling to `0x33000` bytes at `0xc0801000`, BSS to
`0xc0837000..0xc0839000`, and the SRAM stack top to `0xc0840000`.
The raw SPL limit is `0x34f00`; adding the 4 KiB header and 256-byte signature
fits the vendor BootInfo limit of `0x36000`.

DDR v0.2's execution address is `0xc082d000`, as specified by the
[K1 integration patch](https://lists.denx.de/pipermail/u-boot/2026-February/609826.html)
linked from the official firmware repository's README. The older vendor training
code at `0xc0832000` has a different format and ABI and is not interchangeable.
The blob is linked directly at its execution address, with no SRAM copy. Only
its trailing workspace up to BSS is zeroed before training. The linker rejects
resident code/data that would overlap it; with DDR present this leaves 176 KiB
for resident sections, independent of the larger raw-image limit.

Boot, DDR training and FIT loading share a 28 KiB SRAM stack at
`0xc0839000..0xc0840000`, including a 256-byte guard below the usable stack.
`__sstack` is its initial stack pointer; training never clears the live stack.
Fixed placement makes the packed image larger because
it contains the address gap, but removes the duplicate 36,248-byte firmware copy.

Build without DDR firmware:

```
cargo build -p rot-bootloader --release --target riscv64imac-unknown-none-elf --no-default-features --features spacemit-rt/k1-bootrom --bin hello-world
```

Use `--bin eeprom-info` to read the EEPROM without training DDR.
