use std::{ffi::c_void, io, ptr::NonNull};

#[cfg(target_os = "linux")]
use std::{fs::File, os::fd::AsRawFd};

use spacemit_hal::gpio::{k1, k3};

use crate::{RegisterBlock, scan::Soc};

/// Owns a read-only GPIO mapping.
pub(crate) struct Mapping {
    base: NonNull<c_void>,
    offset: usize,
    soc: Soc,
    #[cfg(target_os = "linux")]
    length: usize,
}

impl Mapping {
    /// Maps a GPIO register block read-only.
    ///
    /// # Safety
    /// `address` must identify GPIO with the selected layout, valid and powered
    /// until this mapping is dropped. Keep its Linux driver bound, create no
    /// mutable Rust aliases, and do not access padding or write through this mapping.
    #[cfg(target_os = "linux")]
    pub(crate) unsafe fn new(soc: Soc, address: u64) -> io::Result<Self> {
        // SAFETY: sysconf queries a constant and accesses no Rust memory.
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        if page_size <= 0 || !(page_size as usize).is_power_of_two() || !address.is_multiple_of(4) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid page size or GPIO alignment",
            ));
        }
        let page_address = address & !(page_size as u64 - 1);
        let offset = (address - page_address) as usize;
        let length = offset
            + match soc {
                Soc::K1 => size_of::<k1::RegisterBlock>(),
                Soc::K3 => size_of::<k3::RegisterBlock>(),
            };
        let file_offset = libc::off_t::try_from(page_address).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "GPIO address exceeds off_t")
        })?;
        let file = File::open("/dev/mem")?;
        // SAFETY: The fd is live, the offset is page-aligned, and the mapping is read-only.
        let address = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                length,
                libc::PROT_READ,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                file_offset,
            )
        };
        if address == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        let Some(base) = NonNull::new(address) else {
            // SAFETY: Release the successful mapping, which cannot form Rust references.
            unsafe { libc::munmap(address, length) };
            return Err(io::Error::other("mmap returned address zero"));
        };
        Ok(Self {
            base,
            offset,
            soc,
            length,
        })
    }

    /// Rejects hardware access on unsupported platforms.
    ///
    /// # Safety
    /// This stub creates no mapping and imposes no additional safety requirements.
    #[cfg(not(target_os = "linux"))]
    pub(crate) unsafe fn new(_soc: Soc, _address: u64) -> io::Result<Self> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "requires Linux"))
    }

    /// Borrows the register block for no longer than this mapping.
    pub(crate) fn registers<'a>(&'a self) -> RegisterBlock<'a> {
        // SAFETY: new establishes a live, aligned mapping covering this layout.
        // All accessed registers use interior-mutability wrappers; no padding is
        // accessed and no exclusive reference is created. The returned reference
        // borrows self, so Drop cannot unmap the memory while it is still in use.
        unsafe {
            let base = self.base.as_ptr().cast::<u8>().add(self.offset);
            match self.soc {
                Soc::K1 => RegisterBlock::K1(&*base.cast::<k1::RegisterBlock>()),
                Soc::K3 => RegisterBlock::K3(&*base.cast::<k3::RegisterBlock>()),
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for Mapping {
    fn drop(&mut self) {
        // SAFETY: This owns the mapping, and all register borrows have ended.
        unsafe { libc::munmap(self.base.as_ptr(), self.length) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::ManuallyDrop;

    #[test]
    fn borrows_the_selected_register_block() {
        // SAFETY: All fields are u32 values or transparent register wrappers,
        // so zero is a valid bit pattern for these RAM-backed test blocks.
        let (k1, k3): (k1::RegisterBlock, k3::RegisterBlock) = unsafe { std::mem::zeroed() };
        for (soc, base) in [
            (Soc::K1, NonNull::from(&k1).cast()),
            (Soc::K3, NonNull::from(&k3).cast()),
        ] {
            // Borrow live RAM for this iteration; do not munmap stack storage.
            let mapping = ManuallyDrop::new(Mapping {
                base,
                offset: 0,
                soc,
                #[cfg(target_os = "linux")]
                length: 0,
            });
            match (soc, mapping.registers()) {
                (Soc::K1, RegisterBlock::K1(gpio)) => assert!(std::ptr::eq(gpio, &k1)),
                (Soc::K3, RegisterBlock::K3(gpio)) => assert!(std::ptr::eq(gpio, &k3)),
                _ => panic!("wrong GPIO layout"),
            }
        }
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn rejects_mapping_on_non_linux_hosts() {
        // SAFETY: The non-Linux stub always returns an error without touching hardware.
        let result = unsafe { Mapping::new(Soc::K3, 0) };
        assert!(matches!(result, Err(err) if err.kind() == io::ErrorKind::Unsupported));
    }
}
