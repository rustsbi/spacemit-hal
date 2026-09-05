//! Shared 32-bit, four-byte-stride UART register prefix.

// Linux's 8250 binding pairs spacemit,k1-uart / spacemit,k3-uart with
// intel,xscale-uart, reg-shift = 2, and reg-io-width = 4.
// https://github.com/torvalds/linux/blob/master/Documentation/devicetree/bindings/serial/8250.yaml
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// This prefix does not model scratch/IR registers or XScale IER.UUE (bit 6),
// RTOIE (bit 4), and DMAE (bit 7); it is not a complete XScale initialization API.
// Unlike Allwinner's DesignWare UART, polling here does not use USR at 0x7c.
// K3 pico-ITX's running DT also uses spacemit,k1-uart as its vendor compatible;
// its console at d4017000 has the same xscale fallback, stride, and access width.

/// The 16550-compatible register prefix shared by K1/M1 and K3 UARTs.
pub type RegisterBlock = uart16550::Uart16550<u32>;

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, size_of};

    #[test]
    fn register_layout() {
        // SAFETY: Each field is a transparent UnsafeCell<u32> wrapper.
        let uart: RegisterBlock = unsafe { core::mem::zeroed() };
        let base = &uart as *const _ as usize;
        assert_eq!(uart.rbr_thr() as *const _ as usize - base, 0x00);
        assert_eq!(uart.ier() as *const _ as usize - base, 0x04);
        assert_eq!(uart.iir_fcr() as *const _ as usize - base, 0x08);
        assert_eq!(uart.lcr() as *const _ as usize - base, 0x0c);
        assert_eq!(uart.mcr() as *const _ as usize - base, 0x10);
        assert_eq!(uart.lsr() as *const _ as usize - base, 0x14);
        assert_eq!(uart.msr() as *const _ as usize - base, 0x18);
        assert_eq!(size_of::<RegisterBlock>(), 0x1c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
