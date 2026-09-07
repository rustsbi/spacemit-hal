//! Shared 32-bit, four-byte-stride UART register prefix.

// Linux's 8250 binding pairs spacemit,k1-uart / spacemit,k3-uart with
// intel,xscale-uart, reg-shift = 2, and reg-io-width = 4.
// https://github.com/torvalds/linux/blob/master/Documentation/devicetree/bindings/serial/8250.yaml
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// Unlike Allwinner's DesignWare UART, polling here does not use USR at 0x7c.
// K3 pico-ITX's running DT also uses spacemit,k1-uart as its vendor compatible;
// its console at d4017000 has the same xscale fallback, stride, and access width.

use volatile_register::RW;

/// K1/M1 and K3 UART registers.
#[repr(C)]
pub struct RegisterBlock {
    uart16550: uart16550::Uart16550<u32>,
    /// Scratch register.
    pub scratch: RW<u32>,
}

impl core::ops::Deref for RegisterBlock {
    type Target = uart16550::Uart16550<u32>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.uart16550
    }
}

impl RegisterBlock {
    #[inline]
    pub(super) fn set_divisor(&self, divisor: u16) {
        // uart16550 0.0.1 write_divisor uses non-volatile writes; use volatile aliases.
        // SAFETY: Exclusive mapped UART access; IRQ/DMA are disabled by configure_polling.
        unsafe {
            let lcr = (self.lcr() as *const uart16550::LCR<u32>)
                .cast::<u32>()
                .cast_mut();
            let dll = (self.rbr_thr() as *const uart16550::RBR_THR<u32>)
                .cast::<u32>()
                .cast_mut();
            let dlh = (self.ier() as *const uart16550::IER<u32>)
                .cast::<u32>()
                .cast_mut();
            let line = lcr.read_volatile();
            lcr.write_volatile(line | (1 << 7));
            riscv::asm::fence();
            dll.write_volatile(u32::from(divisor & 0xff));
            dlh.write_volatile(u32::from(divisor >> 8));
            riscv::asm::fence();
            lcr.write_volatile(line & !(1 << 7));
            riscv::asm::fence();
        }
    }
    // The dependency's transparent UnsafeCell<u32> registers permit these
    // overlapping volatile views; extensions do not occupy additional addresses.
    #[inline]
    pub(super) fn configure_polling(&self, line: uart16550::LineControl, parity: u32) {
        self.lcr().write(line);
        // SAFETY: Each pointer targets a live aligned UnsafeCell<u32> register.
        // The caller holds exclusive UART access and line has DLAB cleared.
        unsafe {
            let lcr = (self.lcr() as *const uart16550::LCR<u32>)
                .cast::<u32>()
                .cast_mut();
            // uart16550 0.0.1's PARITY encodings are incorrect; replace bits 3..5.
            lcr.write_volatile((lcr.read_volatile() & !0x38) | parity);
            let ier = (self.ier() as *const uart16550::IER<u32>)
                .cast::<u32>()
                .cast_mut();
            // XScale UUE=1; all interrupt enables and DMAE=0.
            ier.write_volatile(1 << 6);
            let fcr = (self.iir_fcr() as *const uart16550::IIR_FCR<u32>)
                .cast::<u32>()
                .cast_mut();
            // Exact WO command: enable and reset both FIFOs, DMA select=0.
            // uart16550 0.0.1 does not expose the FIFO enable bit.
            fcr.write_volatile(0x07);
        }
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        // SAFETY: Orders device accesses without touching memory or the stack.
        unsafe {
            core::arch::asm!("fence iorw, iorw", options(nostack))
        };
    }
}

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
        assert_eq!(core::mem::offset_of!(RegisterBlock, scratch), 0x1c);
        assert_eq!(size_of::<RegisterBlock>(), 0x20);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
