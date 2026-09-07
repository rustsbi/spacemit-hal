# Architecture link checks

Synthetic link/disassembly fixtures, not board images or QEMU PMU tests.

```sh
cargo rustc -p spacemit-rt --example arch-link \
  --features k3-cpu --target riscv64gc-unknown-none-elf -- \
  -C link-arg=-Tspacemit-rt/tests/arch/link.x
rust-objdump -d --demangle target/riscv64gc-unknown-none-elf/debug/examples/arch-link
```

For A100 (`k3-ai`), add `-C link-arg=--defsym=_boot_hart_id=8`.
`k1-bootrom` and `k3-bootrom` select X60 and X100 entry respectively (hart 0 here).
The runtime owns one 2 KiB boot stack in `.uninit.boot_stack`; secondaries use
application `Stack<N>` allocations and never initialize data/BSS.
K1/K3 PMU release and cross-cluster coherency still require hardware testing.
