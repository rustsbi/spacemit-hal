# `spacemit-rt`

Bare-metal runtime support for SpacemiT K1 and K3 SoCs.

| Cluster feature | Core feature | Bare-metal target |
| --- | --- | --- |
| `k1-cpu` (also M1) | `spacemit-x60` | RV64 |
| `k1-bootrom` (also M1) | `spacemit-x60` | RV64 |
| `k1-mcu` (also M1) | `nuclei-n308` | RV32 |
| `k3-cpu` | `spacemit-x100` | RV64 |
| `k3-ai` | `spacemit-a100` | RV64 |
| `k3-mcu` | `spacemit-rt24` | RV64 |
| `k3-bootrom` | `spacemit-x100` (entry) + `spacemit-a100` | RV64 |

Select at most one cluster/bootrom feature for `_start`; core features may be combined.

## V100 peripherals

`soc::v100::Peripherals` provides ownership tokens for the ACPI-described UART,
12 DesignWare I²C controllers, two supervisor APLIC domains, eight RISC-V IOMMUs,
and IPMI KCS. `take` and `steal` have the same unsafe acquisition contract as
K1/K3. No V100 startup, clock or pin initialization is provided.

Each token has a `BASE: u64` physical address and a checked `ptr()` accessor.
The module is available on RV32 as well; an address above the pointer range
causes `ptr()` to panic rather than truncate. The mappings were collected from
V100 EDK2 ACPI resources (firmware 1.0.0, 2026-06-05). I²C instance numbers follow
ACPI, not Linux adapter numbers. IMSIC hart pages and PCI enumeration are not
modeled by these tokens.

The DesignWare register crate currently comes from the sibling checkout
`../dw-apb-drivers/dw-apb-i2c` (local commit `559bb3c`), pending upstream publication.

## License

This project is licensed under either the MIT license or the Mulan PSL v2.
