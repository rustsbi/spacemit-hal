# rot-bootloader

The board entry reuses `spacemit-rt` startup and initializes MUSE Card M1 devices:

```rust
use rot_bootloader::{Board, entry, println};

#[entry]
fn main(b: Board) {
    println!("PLL1: {:?}", b.clocks.pll1());
}
```

`Board` exposes `eeprom`, `pmic`, and `clocks`. UART0 belongs exclusively to the
global console; other raw peripherals stay inside the entry's static storage.
EEPROM reading and DDR configuration parsing belong to `main`, not the entry.
Initialization failure returns to the runtime without calling `main`.

`println!` writes and flushes on the initializing hart. Calls before console
initialization, on other harts, or during reentrant output are skipped.

Install Rust target:

```
rustup target add riscv64imac-unknown-none-elf
```

Run the example on board:

```
cargo run -p rot-bootloader --release --target riscv64imac-unknown-none-elf
```
