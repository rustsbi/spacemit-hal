# xtask

K1/M1 development FSBL packaging and Fastboot RAM download; no vendor U-Boot, Python or OpenSSL required.

```sh
cargo xtask wrap-fsbl path/to/hello.bin
cargo xtask run path/to/hello.elf
cargo xtask run --serial DEVICE path/to/hello.elf
cargo run -p rot-bootloader --release --target riscv64imac-unknown-none-elf
```

The workspace configures `cargo xtask run` as its RISC-V bare-metal runner; no installation is needed.

Output: `hello-FSBL.bin` beside the input; `run` uses `fastboot devices`, `stage`, then `continue`, never `flash`.

Install [Android SDK Platform-Tools](https://developer.android.com/tools/releases/platform-tools) and add `fastboot` to PATH.

Only the K1/M1 SRAM entry `0xc0801000` is supported; K3 and production secure boot
are not supported. Limits follow vendor `k1_defconfig` and BootInfo: the raw SPL
payload (including address gaps and DDR firmware) must fit `0x34f00` bytes, and
the complete signed container must fit `0x36000` bytes. ELF conversion preserves
fixed-address sections, including DDR v0.2 firmware at `0xc082d000`.

After building the DDR-enabled example, verify ELF conversion, the single firmware
copy, its execution address and signed-container size without accessing hardware:

```powershell
$env:SPACEMIT_LAYOUT_ELF = (Resolve-Path 'target/riscv64imac-unknown-none-elf/release/rot-bootloader').Path
cargo test -p xtask linked_ddr_image -- --ignored
```

Build a complete NOR image from a verified, board-specific 8-MiB backup:

```powershell
cargo xtask pack-nor --backup original-nor.bin --fsbl FSBL.bin --output nor.bin
```

Only the FSBL slot (`0x20000..0x60000`) changes by default; bootinfo, private data,
environment and later images are preserved. Optional `--sbi` and `--payload`
replace the corresponding FIT containers after validation. The environment at
`0x60000` accepts 16-KiB and legacy 64-KiB CRC formats; `mtdparts` selects the
image partitions. Invalid metadata or images abort packaging. Existing output
files are never overwritten; `pack-nor` does not access hardware.
