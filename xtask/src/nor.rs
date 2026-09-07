use crate::{
    fsbl,
    image::{
        self,
        layout::{ENV_MAX_SIZE, ENV_OFFSET, Layout, Partition, crc32},
    },
};
use anyhow::{Context, Result, anyhow, ensure};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};

pub fn pack(
    backup: &Path,
    fsbl: &Path,
    sbi: Option<&Path>,
    payload: Option<&Path>,
    output: &Path,
) -> Result<()> {
    let read = |path: &Path| fs::read(path).with_context(|| format!("read {}", path.display()));
    let backup_bytes = read(backup)?;
    let fsbl_bytes = read(fsbl)?;
    let sbi_bytes = sbi.map(read).transpose()?;
    let payload_bytes = payload.map(read).transpose()?;
    let result = build(
        &backup_bytes,
        &fsbl_bytes,
        sbi_bytes.as_deref(),
        payload_bytes.as_deref(),
    )?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .with_context(|| format!("create {} (must not already exist)", output.display()))?;
    file.write_all(&result)?;
    file.sync_all()?;
    println!(
        "{}: {} bytes; SHA256 {:x}",
        output.display(),
        result.len(),
        Sha256::digest(&result)
    );
    println!(
        "Backup SHA256 {:x}; bootinfo, private data and untouched partitions retained",
        Sha256::digest(&backup_bytes)
    );
    Ok(())
}

fn build(
    backup: &[u8],
    fsbl: &[u8],
    sbi: Option<&[u8]>,
    payload: Option<&[u8]>,
) -> Result<Vec<u8>> {
    ensure!(
        backup.len() == 0x800000,
        "MUSE Card requires a complete 8-MiB NOR backup"
    );
    let mut output = backup.to_vec();
    let env = &output[ENV_OFFSET as usize..ENV_OFFSET as usize + ENV_MAX_SIZE];
    let layout =
        Layout::from_env(env, backup.len() as u32).map_err(|e| anyhow!("environment: {e:?}"))?;
    // Vendor bootinfo v1.1: a 64-byte header followed by its little-endian CRC32.
    let le = |at| u32::from_le_bytes(backup[at..at + 4].try_into().unwrap());
    ensure!(
        le(0) == 0xb00714f0 && le(4) == 0x10001 && &backup[8..12] == b"NORF",
        "invalid K1 NOR bootinfo"
    );
    ensure!(crc32(&backup[..64]) == le(64), "bootinfo CRC mismatch");
    ensure!(
        le(32) == layout.fsbl.offset,
        "bootinfo SPL0 differs from fsbl partition"
    );
    ensure!(
        fsbl.len() <= le(40) as usize,
        "FSBL exceeds BootROM load limit"
    );
    fsbl::validate(fsbl)?;
    replace(&mut output, layout.fsbl, fsbl)?;
    if let Some(sbi) = sbi {
        replace(&mut output, layout.sbi, sbi)?;
    }
    if let Some(payload) = payload {
        replace(&mut output, layout.payload, payload)?;
    }
    let images = image::inspect(fit(&output, layout.sbi)?, fit(&output, layout.payload)?)
        .map_err(|e| anyhow!("FIT verification: {e:?}"))?
        .destinations();
    println!(
        "NOR layout: FSBL {:#x}+{:#x}, SBI {:#x}+{:#x}, next stage {:#x}+{:#x}",
        layout.fsbl.offset,
        layout.fsbl.size,
        layout.sbi.offset,
        layout.sbi.size,
        layout.payload.offset,
        layout.payload.size
    );
    println!(
        "Verified destinations: SBI {:#x}, next stage {:#x}, DTB {:#x}",
        images.sbi.address, images.payload.address, images.dtb.address
    );
    Ok(output)
}

fn replace(output: &mut [u8], partition: Partition, bytes: &[u8]) -> Result<()> {
    ensure!(
        !bytes.is_empty() && bytes.len() <= partition.size as usize,
        "image exceeds NOR partition"
    );
    let start = partition.offset as usize;
    let region = output
        .get_mut(start..start + partition.size as usize)
        .context("partition exceeds NOR")?;
    region.fill(0xff);
    region[..bytes.len()].copy_from_slice(bytes);
    Ok(())
}

fn fit(bytes: &[u8], partition: Partition) -> Result<&[u8]> {
    let start = partition.offset as usize;
    let region = bytes
        .get(start..start + partition.size as usize)
        .context("partition exceeds NOR")?;
    ensure!(
        region.len() >= 40 && region[..4] == [0xd0, 0x0d, 0xfe, 0xed],
        "missing FIT at {start:#x}"
    );
    let size = u32::from_be_bytes(region[4..8].try_into()?) as usize;
    ensure!(size >= 40, "invalid FIT length");
    region.get(..size).context("FIT exceeds NOR partition")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_incomplete_backup_and_invalid_environment() {
        assert!(build(&[0; 80], &[0; 32], None, None).is_err());
        assert!(build(&vec![0xff; 0x800000], &[0; 32], None, None).is_err());
        assert!(replace(&mut [0; 8], Partition { offset: 4, size: 4 }, &[0; 5]).is_err());
    }

    #[test]
    fn environment_formats_and_corruption() {
        use image::layout::{ENV_SIZE, Error};
        let parts = b"mtdparts=d420c000.spi-0:64K(bootinfo),64K(private),256K(fsbl),64K(env),512K(opensbi),-(uboot)\0\0";
        for size in [ENV_SIZE, ENV_MAX_SIZE] {
            let mut env = vec![0xff; ENV_MAX_SIZE];
            env[4..4 + parts.len()].copy_from_slice(parts);
            let crc = crc32(&env[4..size]);
            env[..4].copy_from_slice(&crc.to_le_bytes());
            let layout = Layout::from_env(&env, 0x800000).unwrap();
            assert_eq!(layout.env_size, size);
            assert_eq!(layout.sbi.offset, 0x70000);
            assert_eq!(layout.payload.offset, 0xf0000);
            env[8] ^= 1;
            assert_eq!(Layout::from_env(&env, 0x800000), Err(Error::Integrity));
        }
    }

    #[test]
    #[ignore = "set SPACEMIT_NOR_BACKUP and SPACEMIT_FSBL to local verified board images"]
    fn board_backup_preservation_and_rejection() {
        let backup = fs::read(std::env::var_os("SPACEMIT_NOR_BACKUP").unwrap()).unwrap();
        let fsbl = fs::read(std::env::var_os("SPACEMIT_FSBL").unwrap()).unwrap();
        let output = build(&backup, &fsbl, None, None).unwrap();
        assert_eq!(&output[..0x20000], &backup[..0x20000]);
        assert_eq!(&output[0x60000..], &backup[0x60000..]);
        assert_eq!(&output[0x20000..0x20000 + fsbl.len()], &fsbl);
        assert!(
            output[0x20000 + fsbl.len()..0x60000]
                .iter()
                .all(|&b| b == 0xff)
        );
        for offset in [0, 64, ENV_OFFSET as usize + 4, 0x70000, 0xf0000] {
            let mut corrupted = backup.clone();
            corrupted[offset] ^= 1;
            assert!(build(&corrupted, &fsbl, None, None).is_err());
        }
        let mut corrupted = fsbl.clone();
        corrupted[0x1000] ^= 1;
        assert!(build(&backup, &corrupted, None, None).is_err());
    }
}
