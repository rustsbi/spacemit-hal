# qspi-nor

Allocation-free, blocking NOR reads over `backend::Backend`.

`NorFlash::probe` reads JEDEC/SFDP and selects 03h reads for flashes up to 16 MiB already in single-line, three-byte-address mode; `NorFlash::new` accepts board-verified parameters for other read protocols without configuring QE, QPI, bank or address-mode registers.

Implements `embedded_storage::nor_flash::ReadNorFlash`; no program, erase, XIP or heap allocation.
