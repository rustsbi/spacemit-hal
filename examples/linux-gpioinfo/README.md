# linux-gpioinfo

Scans SpacemiT GPIO device-tree nodes and reads their direction registers.

```sh
cargo build --release -p linux-gpioinfo
target/release/linux-gpioinfo scan
sudo target/release/linux-gpioinfo read
```

The output will be:

```
$ target/release/linux-gpioinfo scan
K3: /proc/device-tree/soc/gpio@d4019000 reg base = 0xd4019000
$ sudo target/release/linux-gpioinfo read
K3: /proc/device-tree/soc/gpio@d4019000 reg base = 0xd4019000
gpio0: 0x400c8000
gpio1: 0x00000008
gpio2: 0x00000000
gpio3: 0x00400020
```

`scan` lists K1/K3 nodes under `/proc/device-tree/soc/gpio@*` and their `reg` bases.
`read` (default) uses read-only `mmap` and HAL registers to print four `u32` values
for GPIO0..3: each bit is `0` = input, `1` = output.
