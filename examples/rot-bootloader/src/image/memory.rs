use super::{Error, Result};

// Physical address zero is valid DRAM for K1's SBI but cannot form a Rust reference.
pub(super) unsafe fn copy(address: usize, data: &[u8]) -> Result<()> {
    #[cfg(target_arch = "riscv64")]
    {
        let mut offset = 0;
        // SAFETY: The caller supplies disjoint, bounded physical DRAM and verified source bytes.
        unsafe {
            if address.is_multiple_of(8) {
                for word in data.as_chunks::<8>().0 {
                    let value = u64::from_le_bytes(*word);
                    core::arch::asm!("sd {value}, 0({address})", value = in(reg) value,
                        address = in(reg) address + offset, options(nostack));
                    offset += 8;
                }
            }
            for &byte in &data[offset..] {
                core::arch::asm!("sb {value}, 0({address})", value = in(reg) byte as usize,
                    address = in(reg) address + offset, options(nostack));
                offset += 1;
            }
            riscv::asm::fence();
            for line in (address & !63..(address + data.len() + 63) & !63).step_by(64) {
                core::arch::asm!(
                    ".option push
                    .option arch, +zicbom
                    cbo.flush ({address})
                    .option pop",
                    address = in(reg) line, options(nostack),
                );
            }
            riscv::asm::fence();
            // Read after writeback/invalidation so corruption is not hidden by dirty cache lines.
            for (offset, &expected) in data.iter().enumerate() {
                let actual: usize;
                core::arch::asm!("lbu {value}, 0({address})", value = out(reg) actual,
                    address = in(reg) address + offset, options(nostack, readonly));
                if actual != usize::from(expected) {
                    return Err(Error::Readback);
                }
            }
        }
        riscv::asm::fence_i();
        Ok(())
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (address, data);
        Err(Error::Unsupported)
    }
}
