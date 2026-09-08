use anyhow::{Result, ensure};
use object::read::elf::{ElfFile64, ProgramHeader};
use object::{Object, ObjectSection, SectionFlags, SectionKind};

pub const BASE: u64 = 0xc080_1000;
// Vendor k1_defconfig CONFIG_SPL_SIZE_LIMIT, including appended DDR firmware/DTB.
pub const LIMIT: usize = 0x34f00;

// Like allwinner-hal/rfel: copy allocated, file-backed sections and preserve address gaps.
pub fn convert(bytes: &[u8]) -> Result<Vec<u8>> {
    let elf = object::File::parse(bytes)?;
    ensure!(
        elf.format() == object::BinaryFormat::Elf
            && elf.architecture() == object::Architecture::Riscv64
            && elf.is_little_endian()
            && elf.kind() == object::ObjectKind::Executable,
        "expected a little-endian RISC-V64 executable ELF"
    );
    ensure!(elf.entry() == BASE, "expected K1/M1 entry at {BASE:#x}");
    let headers = ElfFile64::<object::Endianness>::parse(bytes)?;
    for segment in headers.elf_program_headers() {
        let endian = headers.endian();
        if segment.p_type(endian) == object::elf::PT_LOAD {
            ensure!(
                segment.p_vaddr(endian) == segment.p_paddr(endian),
                "relocated ELF sections are not supported"
            );
        }
    }
    let mut sections = Vec::new();
    for section in elf.sections() {
        if matches!(section.flags(), SectionFlags::Elf { sh_flags, .. } if sh_flags.intersects(object::elf::SHF_ALLOC))
            && section.kind() != SectionKind::UninitializedData
            && section.size() != 0
        {
            let data = section.data()?;
            ensure!(
                data.len() as u64 == section.size(),
                "unsupported compressed section"
            );
            sections.push((section.address(), data));
        }
    }
    sections.sort_by_key(|(address, _)| *address);
    ensure!(
        sections.first().map(|s| s.0) == Some(BASE),
        "missing SRAM entry section"
    );
    let mut raw = Vec::new();
    for (address, data) in sections {
        let offset = address
            .checked_sub(BASE)
            .ok_or_else(|| anyhow::anyhow!("section below SRAM"))?;
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or_else(|| anyhow::anyhow!("section overflow"))?;
        ensure!(
            end <= LIMIT as u64 && offset >= raw.len() as u64,
            "overlapping or oversized SRAM sections"
        );
        raw.resize(end as usize, 0);
        raw[offset as usize..].copy_from_slice(data);
    }
    Ok(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        let mut elf = vec![0; 0x300];
        elf[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        elf[16..18].copy_from_slice(&2u16.to_le_bytes());
        elf[18..20].copy_from_slice(&243u16.to_le_bytes());
        elf[20..24].copy_from_slice(&1u32.to_le_bytes());
        elf[24..32].copy_from_slice(&BASE.to_le_bytes());
        elf[40..48].copy_from_slice(&0x100u64.to_le_bytes());
        elf[52..54].copy_from_slice(&64u16.to_le_bytes());
        elf[58..60].copy_from_slice(&64u16.to_le_bytes());
        elf[60..62].copy_from_slice(&5u16.to_le_bytes());
        elf[62..64].copy_from_slice(&4u16.to_le_bytes());
        elf[0x204..0x208].copy_from_slice(&3u32.to_le_bytes());
        elf[0x218..0x220].copy_from_slice(&0x290u64.to_le_bytes());
        elf[0x220..0x228].copy_from_slice(&1u64.to_le_bytes());
        for (i, address, kind, offset) in [
            (1, BASE, 1u32, 0x280u64),
            (2, BASE + 8, 1, 0x284),
            (3, BASE + 0x30000, 8, 0),
        ] {
            let header = &mut elf[0x100 + i * 64..0x140 + i * 64];
            header[4..8].copy_from_slice(&kind.to_le_bytes());
            header[8..16].copy_from_slice(&2u64.to_le_bytes());
            header[16..24].copy_from_slice(&address.to_le_bytes());
            header[24..32].copy_from_slice(&offset.to_le_bytes());
            header[32..40].copy_from_slice(&4u64.to_le_bytes());
        }
        elf[0x280..0x288].copy_from_slice(b"abcdefgh");
        elf
    }

    #[test]
    fn preserves_gaps_and_skips_bss() {
        assert_eq!(convert(&fixture()).unwrap(), b"abcd\0\0\0\0efgh");
    }

    #[test]
    fn preserves_fixed_ddr_address_and_accepts_exact_limit() {
        for offset in [0x2c000, LIMIT - 4] {
            let mut elf = fixture();
            elf[0x190..0x198].copy_from_slice(&(BASE + offset as u64).to_le_bytes());
            let raw = convert(&elf).unwrap();
            assert_eq!(raw.len(), offset + 4);
            assert_eq!(&raw[..4], b"abcd");
            assert!(raw[4..offset].iter().all(|&byte| byte == 0));
            assert_eq!(&raw[offset..], b"efgh");
        }
    }

    #[test]
    fn rejects_bad_entry_overlap_and_size() {
        let mut elf = fixture();
        elf[24] ^= 1;
        assert!(convert(&elf).is_err());
        for address in [BASE, BASE + LIMIT as u64] {
            let mut elf = fixture();
            elf[0x190..0x198].copy_from_slice(&address.to_le_bytes());
            assert!(convert(&elf).is_err());
        }
    }

    #[test]
    fn rejects_non_elf() {
        assert!(super::convert(b"not an ELF").is_err());
    }

    #[test]
    #[ignore = "set SPACEMIT_LAYOUT_ELF to a linked DDR-enabled rot-bootloader ELF"]
    fn linked_ddr_image_has_one_firmware_at_its_execution_address() {
        let path = std::env::var_os("SPACEMIT_LAYOUT_ELF").unwrap();
        let bytes = std::fs::read(path).unwrap();
        let elf = object::File::parse(bytes.as_slice()).unwrap();
        let info = elf.section_by_name(".ddr_info").unwrap();
        assert_eq!((info.address(), info.size()), (0xc0800000, 1536));
        assert_eq!(info.kind(), SectionKind::UninitializedData);
        let stack = elf.section_by_name(".stack").unwrap();
        assert_eq!((stack.address(), stack.size()), (0xc0839000, 0x7000));
        assert_eq!(stack.kind(), SectionKind::UninitializedData);
        assert!(elf.section_by_name(".ddr_stack").is_none());
        let raw = convert(&bytes).unwrap();
        let blob = std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../vendor/spacemit-firmware/k1/v0.2/ddr_fw.bin"),
        )
        .unwrap();
        let offset = (0xc082_d000 - BASE) as usize;
        assert_eq!(&raw[offset..], blob);
        assert_eq!(
            raw.windows(blob.len())
                .filter(|window| *window == blob)
                .count(),
            1
        );
        let image = crate::fsbl::wrap(&raw).unwrap();
        assert_eq!(&image[0x1000 + offset..0x1000 + raw.len()], blob);
        assert!(image.len() <= 0x36000);
    }
}
