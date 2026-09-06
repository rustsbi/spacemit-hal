# rot-bootloader

Install Rust target:

```
rustup target add riscv64imac-unknown-none-elf
```

Run the example on board:

```
cargo run -p rot-bootloader --release --target riscv64imac-unknown-none-elf
```
