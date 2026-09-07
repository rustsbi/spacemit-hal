//! Single-hart K1 dynamic SBI handoff; SBI owns subsequent hart startup.

use crate::image::LoadedImages;

/// Version 2 dynamic SBI boot parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DynamicInfo {
    pub magic: usize,
    pub version: usize,
    pub next_addr: usize,
    pub next_mode: usize,
    pub options: usize,
    pub boot_hart: usize,
}

impl DynamicInfo {
    /// Selects S-mode and hart 0 without firmware-specific options.
    pub const fn new(next_addr: usize) -> Self {
        Self {
            magic: 0x4942534f,
            version: 2,
            next_addr,
            next_mode: 1,
            options: 0,
            boot_hart: 0,
        }
    }
}

const _: () = assert!(core::mem::size_of::<DynamicInfo>() == 6 * core::mem::size_of::<usize>());

#[cfg(target_arch = "riscv64")]
#[repr(C, align(64))]
struct Parameters(DynamicInfo);

#[cfg(target_arch = "riscv64")]
static mut PARAMETERS: Parameters = Parameters(DynamicInfo::new(0));

/// Transfers hart 0 to validated SBI firmware in M-mode.
///
/// # Safety
/// Run once on K1/M1 hart 0 after successful image loading, with no active DMA.
///
/// Other harts must remain powered off; the SBI firmware must own their PMU/HSM startup.
///
/// Loaded memory and SRAM must remain accessible; no Rust code or peripheral owner may resume.
pub unsafe fn enter(images: LoadedImages) -> ! {
    #[cfg(target_arch = "riscv64")]
    {
        if riscv::register::mhartid::read() != 0 {
            // SAFETY: This entry is M-mode; a non-boot hart must never inherit the platform.
            unsafe { spacemit_rt::halt() };
        }
        let parameters = core::ptr::addr_of_mut!(PARAMETERS);
        // SAFETY: The sole boot hart owns this cache-line-aligned SRAM object until handoff.
        unsafe { parameters.write(Parameters(DynamicInfo::new(images.payload_entry))) };
        crate::eprintln!(
            "SBI handoff: hart=0 entry={:#x} dtb={:#x} next={:#x}; secondary harts stay off",
            images.sbi_entry,
            images.dtb.address,
            images.payload_entry
        );
        // Loaded images were cleaned and read back; publish the remaining parameter cache line.
        // Keep X60 caches and CCI enabled, as vendor SPL does; SBI initializes its own contexts.
        // SAFETY: All UART output is flushed, QSPI PIO has completed, and this assembly never returns.
        unsafe {
            core::arch::asm!(
                ".option push
                .option norelax
                li      t0, 0x2000a
                csrc    mstatus, t0
                csrw    mie, zero
                csrw    mideleg, zero
                csrw    medeleg, zero
                csrci   mip, 2
                csrw    satp, zero
                sfence.vma
                csrw    mscratch, zero
                csrw    sscratch, zero
                lla     t0, 2f
                csrw    mtvec, t0
                mv      ra, t0
                fence   iorw, iorw
                .option arch, +zicbom
                cbo.flush (a2)
                fence   iorw, iorw
                fence.i
                jr      a3
                .balign 4
            2:  csrci   mstatus, 8
                csrw    mie, zero
            3:  wfi
                j       3b
                .option pop",
                in("a0") 0usize,
                in("a1") images.dtb.address,
                in("a2") parameters,
                in("a3") images.sbi_entry,
                options(noreturn),
            );
        }
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = images;
        unimplemented!("K1 handoff requires RV64 M-mode");
    }
}
