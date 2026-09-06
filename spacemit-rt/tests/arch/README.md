# Architecture link check

This fixture is for linking and disassembly only, not hardware booting.
It links the feature-selected root entry and uses distinct data
load/run addresses, odd-length objects and separate aligned hart stacks.
Data/BSS boundaries and the data load address must be aligned to XLEN / 8;
this fixture pads both sections to eight bytes for RV32 and RV64 word loops.

```sh
cargo rustc -p spacemit-rt --example arch-link \
  --features k3-cpu --target riscv64gc-unknown-none-elf -- \
  -C link-arg=-Tspacemit-rt/tests/arch/link.x
rust-objdump -d --demangle target/riscv64gc-unknown-none-elf/debug/examples/arch-link
```

`_max_hart_id` and `_hart_stack_size` configure the stack slots; `_boot_hart_id`
selects the shared-memory initializer (use 8 for A100). The loader must load the
zero-valued `.boot_sync` section at its runtime address before releasing harts.
Secondary harts poll the shared ready flag with acquire ordering; no interrupt
controller or wakeup hooks are required. Every hart enters Rust on its own stack.
`_delay_hart_id` defaults to the boot hart; override it with a secondary hart ID
to test entry after the boot hart has already initialized memory.

The `multihart` fixture runs the RT24 standard-CSR path on QEMU virt, not K3 hardware:

```sh
cargo rustc -p spacemit-rt --example multihart --features k3-mcu \
  --target riscv64gc-unknown-none-elf -- \
  -C link-arg=--defsym=_stext=0x80000000 \
  -C link-arg=--defsym=_max_hart_id=3 \
  -C link-arg=-Tspacemit-rt/tests/arch/link.x
qemu-system-riscv64 -machine virt -smp 4 -display none -serial none -monitor none \
  -bios target/riscv64gc-unknown-none-elf/debug/examples/multihart
```
