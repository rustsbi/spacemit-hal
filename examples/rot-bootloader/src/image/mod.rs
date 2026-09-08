//! Official K1 NOR FIT loading with CRC32 integrity checks.

mod fdt;
pub mod layout;
mod memory;

use embedded_storage::nor_flash::ReadNorFlash;
use fdt::{Fdt, Node, integer, text, word};
use layout::{ENV_MAX_SIZE, ENV_OFFSET, Layout, crc32};

const SBI_LIMIT: usize = 0x0008_0000;
const PAYLOAD_LIMIT: usize = 0x0080_0000;
const STAGING_START: usize = 0x0100_0000;
const STAGING_SIZE: usize = SBI_LIMIT + PAYLOAD_LIMIT;

type Result<T> = core::result::Result<T, Error>;

/// NOR initialization or image loading failed.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The NOR environment or partition layout is invalid.
    Environment(layout::Error),
    /// QSPI initialization failed.
    Qspi(spacemit_hal::qspi::Error),
    /// A NOR operation failed.
    Nor(qspi_nor::Error<spacemit_hal::qspi::Error>),
    /// An image or FDT structure is malformed.
    Format,
    /// A required image, property or board configuration is missing.
    Missing,
    /// The image encoding, compression or hash algorithm is unsupported.
    Unsupported,
    /// A size or offset exceeds its permitted region.
    Bounds,
    /// A payload load or entry address is invalid.
    Address,
    /// Loaded images overlap.
    Overlap,
    /// A CRC32 integrity check failed.
    Integrity,
    /// DRAM readback differs from the verified image.
    Readback,
    /// Image loading exhausted the application stack guard.
    Stack,
}

impl From<spacemit_hal::qspi::Error> for Error {
    fn from(error: spacemit_hal::qspi::Error) -> Self {
        Self::Qspi(error)
    }
}

impl From<qspi_nor::Error<spacemit_hal::qspi::Error>> for Error {
    fn from(error: qspi_nor::Error<spacemit_hal::qspi::Error>) -> Self {
        Self::Nor(error)
    }
}

/// A validated physical image destination.
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    /// Physical DRAM load address.
    pub address: usize,
    /// Image length in bytes.
    pub size: usize,
}

impl Segment {
    fn end(self) -> Result<usize> {
        self.address.checked_add(self.size).ok_or(Error::Bounds)
    }

    fn within(self, start: usize, end: usize) -> Result<()> {
        if self.size == 0 || self.address < start || self.end()? > end {
            return Err(Error::Address);
        }
        Ok(())
    }
}

/// Verified SBI, next-stage and DTB destinations without a firmware handoff.
#[derive(Debug)]
pub struct LoadedImages {
    /// SBI code and data.
    pub sbi: Segment,
    /// Next-stage code and data.
    pub payload: Segment,
    /// Board device tree.
    pub dtb: Segment,
    /// SBI entry address.
    pub sbi_entry: usize,
    /// Next-stage entry address.
    pub payload_entry: usize,
}

struct Image<'a> {
    data: &'a [u8],
    destination: Segment,
    entry: usize,
}

/// Validated image bytes retained until copying finishes.
pub struct Images<'a> {
    sbi: Image<'a>,
    payload: Image<'a>,
    dtb: Image<'a>,
}

impl Images<'_> {
    /// Returns physical destinations without accessing hardware.
    pub fn destinations(&self) -> LoadedImages {
        LoadedImages {
            sbi: self.sbi.destination,
            payload: self.payload.destination,
            dtb: self.dtb.destination,
            sbi_entry: self.sbi.entry,
            payload_entry: self.payload.entry,
        }
    }
}

/// Validates vendor FIT images using the EEPROM product or FIT default.
pub fn inspect<'a>(
    sbi: &'a [u8],
    payload: &'a [u8],
    product_name: Option<&str>,
) -> Result<Images<'a>> {
    if sbi.len() > SBI_LIMIT || payload.len() > PAYLOAD_LIMIT {
        return Err(Error::Bounds);
    }
    let sbi_root = Fdt::root(sbi)?;
    let configurations = sbi_root.child("configurations")?;
    let selected = configurations.child(text(configurations.required("default")?)?)?;
    let sbi_node = sbi_root
        .child("images")?
        .child(text(selected.required("firmware")?)?)?;
    check(sbi_node, "type", "firmware")?;
    // Vendor SPL uses this tag for the dynamic SBI handoff, including compatible RustSBI firmware.
    check(sbi_node, "os", "opensbi")?;
    check(sbi_node, "arch", "riscv")?;
    let sbi = read_image(sbi_node, None)?;
    // Reserve the complete low 2 MiB for SBI firmware, including its own BSS/scratch.
    sbi.destination.within(0, 0x0020_0000)?;
    check_entry(&sbi)?;

    let root = Fdt::root(payload)?;
    let configurations = root.child("configurations")?;
    let configuration = if let Some(product_name) = product_name {
        let mut selected = None;
        for node in configurations.children() {
            let node = node?;
            if node.property("description")?.map(text).transpose()? == Some(product_name)
                && selected.replace(node).is_some()
            {
                return Err(Error::Format);
            }
        }
        selected.ok_or(Error::Missing)?
    } else {
        configurations.child(text(configurations.required("default")?)?)?
    };
    let images = root.child("images")?;
    let payload_node = images.child(text(configuration.required("loadables")?)?)?;
    check(payload_node, "type", "standalone")?;
    check(payload_node, "os", "U-Boot")?;
    check(payload_node, "arch", "riscv")?;
    let payload = read_image(payload_node, None)?;
    payload.destination.within(0x0020_0000, STAGING_START)?;
    check_entry(&payload)?;

    let dtb_node = images.child(text(configuration.required("fdt")?)?)?;
    check(dtb_node, "type", "flat_dt")?;
    // Vendor spl_fit_append_fdt appends an unaddressed DTB after u-boot-nodtb.bin.
    let appended = payload
        .destination
        .end()?
        .checked_add(7)
        .ok_or(Error::Bounds)?
        & !7;
    let dtb = read_image(dtb_node, Some(appended))?;
    dtb.destination.within(0x0020_0000, STAGING_START)?;
    if !dtb.destination.address.is_multiple_of(8)
        || dtb.data.len() > 0x40000
        || Fdt::size(dtb.data)? != dtb.data.len()
    {
        return Err(Error::Bounds);
    }
    Fdt::root(dtb.data)?;
    let segments = [sbi.destination, payload.destination, dtb.destination];
    for (index, first) in segments.iter().enumerate() {
        for second in &segments[index + 1..] {
            if first.address < second.end()? && second.address < first.end()? {
                return Err(Error::Overlap);
            }
        }
    }
    Ok(Images { sbi, payload, dtb })
}

fn check(node: Node<'_>, name: &str, expected: &str) -> Result<()> {
    if text(node.required(name)?)? != expected {
        return Err(Error::Unsupported);
    }
    Ok(())
}

fn check_entry(image: &Image<'_>) -> Result<()> {
    if !image.entry.is_multiple_of(2)
        || image.entry < image.destination.address
        || image.entry >= image.destination.end()?
    {
        return Err(Error::Address);
    }
    Ok(())
}

fn read_image(node: Node<'_>, fallback: Option<usize>) -> Result<Image<'_>> {
    check(node, "compression", "none")?;
    // The inspected official FITs use inline data, not external data or overlays.
    for name in ["data-offset", "data-position", "data-size"] {
        if node.property(name)?.is_some() {
            return Err(Error::Unsupported);
        }
    }
    let data = node.required("data")?;
    let mut verified = false;
    for hash in node.children() {
        let hash = hash?;
        if hash.name.starts_with("hash") {
            check(hash, "algo", "crc32")?;
            if word(hash.required("value")?)? != crc32(data) {
                return Err(Error::Integrity);
            }
            verified = true;
        }
    }
    if !verified {
        return Err(Error::Missing);
    }
    let address = node
        .property("load")?
        .map(integer)
        .transpose()?
        .or(fallback)
        .ok_or(Error::Missing)?;
    let entry = node
        .property("entry")?
        .map(integer)
        .transpose()?
        .unwrap_or(address);
    Ok(Image {
        data,
        destination: Segment {
            address,
            size: data.len(),
        },
        entry,
    })
}

// Caller reserves trained DRAM 0..128 MiB and excludes all external users.
pub(crate) unsafe fn load(
    flash: &mut impl ReadNorFlash<Error = qspi_nor::Error<spacemit_hal::qspi::Error>>,
    product_name: Option<&str>,
) -> Result<LoadedImages> {
    // SAFETY: Staging is disjoint from validated destinations and all SRAM allocations.
    let staging = unsafe {
        core::ptr::write_bytes(STAGING_START as *mut u8, 0, STAGING_SIZE);
        core::slice::from_raw_parts_mut(STAGING_START as *mut u8, STAGING_SIZE)
    };
    flash.read(ENV_OFFSET, &mut staging[..ENV_MAX_SIZE])?;
    let layout = Layout::from_env(&staging[..ENV_MAX_SIZE], flash.capacity() as u32)
        .map_err(Error::Environment)?;
    crate::eprintln!(
        "NOR environment: {:#x} bytes, CRC32 verified",
        layout.env_size
    );
    crate::eprintln!(
        "NOR partitions: SBI {:#x}+{:#x}, next stage {:#x}+{:#x}",
        layout.sbi.offset,
        layout.sbi.size,
        layout.payload.offset,
        layout.payload.size
    );
    let (sbi, payload) = staging.split_at_mut(SBI_LIMIT);
    let sbi = read_fit(
        flash,
        layout.sbi.offset,
        &mut sbi[..SBI_LIMIT.min(layout.sbi.size as usize)],
    )?;
    let payload = read_fit(
        flash,
        layout.payload.offset,
        &mut payload[..PAYLOAD_LIMIT.min(layout.payload.size as usize)],
    )?;
    let images = inspect(sbi, payload, product_name)?;
    for image in [&images.sbi, &images.payload, &images.dtb] {
        // SAFETY: All destinations were bounded, checked for overlap and verified before any copy.
        unsafe { memory::copy(image.destination.address, image.data)? };
    }
    Ok(images.destinations())
}

// The caller owns 256 initialized bytes at the stack's lower boundary.
pub(crate) unsafe fn check_stack_guard(bottom: usize) -> Result<()> {
    for offset in 0..256 {
        // SAFETY: The caller reserves this initialized guard, without concurrent writers.
        if unsafe { core::ptr::read_volatile((bottom + offset) as *const u8) } != 0xa5 {
            return Err(Error::Stack);
        }
    }
    Ok(())
}

fn read_fit<'a>(
    flash: &mut impl ReadNorFlash<Error = qspi_nor::Error<spacemit_hal::qspi::Error>>,
    offset: u32,
    buffer: &'a mut [u8],
) -> Result<&'a [u8]> {
    let available = flash
        .capacity()
        .checked_sub(offset as usize)
        .ok_or(Error::Bounds)?;
    if available < 40 || buffer.len() < 40 {
        return Err(Error::Bounds);
    }
    flash.read(offset, &mut buffer[..40])?;
    let size = Fdt::size(&buffer[..40])?;
    if size > available || size > buffer.len() {
        return Err(Error::Bounds);
    }
    flash.read(offset + 40, &mut buffer[40..size])?;
    Ok(&buffer[..size])
}
