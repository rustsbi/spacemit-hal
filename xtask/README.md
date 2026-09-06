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

Only the K1/M1 SRAM entry `0xc0801000` and 128 KiB image region are supported; K3 and production secure boot are not supported.
