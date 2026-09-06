use anyhow::{Result, ensure};
use object::read::elf::{ElfFile64, ProgramHeader};
use object::{Object, ObjectSection, SectionFlags, SectionKind};

pub const BASE: u64 = 0xc080_1000;
pub const LIMIT: usize = 0x20000;

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
}
