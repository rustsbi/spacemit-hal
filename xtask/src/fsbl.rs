use anyhow::{Result, ensure};
use rsa::{
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey,
    traits::PublicKeyParts,
};
use sha2::{Digest, Sha256};

// K1 vendor board/spacemit/k1-x/configs/fsbl.json; secure=0 development container.
pub fn wrap(raw: &[u8]) -> Result<Vec<u8>> {
    ensure!(
        !raw.is_empty() && raw.len() <= crate::elf2bin::LIMIT,
        "FSBL must contain 1..=128 KiB"
    );
    let root = RsaPrivateKey::from_pkcs1_pem(include_str!("../keys/rsakeypair0_prv.key"))?;
    let spl = RsaPrivateKey::from_pkcs1_pem(include_str!("../keys/spl_pubkey_prv.key"))?;
    let size = raw.len().next_multiple_of(32);
    let mut image = vec![0; 0x1000 + size];
    image[..256].copy_from_slice(&root.n().to_bytes_be());
    header(&mut image[0x100..0x120], 0x1000);
    image[0x124..0x128].copy_from_slice(&4u32.to_le_bytes());
    for (i, name) in ["spl", "uboot", "kernel", "rootfs"].iter().enumerate() {
        let offset = 0x128 + i * 20;
        image[offset..offset + name.len()].copy_from_slice(name.as_bytes());
        image[offset + 16..offset + 20].copy_from_slice(&(i as u32).to_le_bytes());
    }
    let hash = Sha256::digest(&image[..256]);
    image[0x2b8..0x2d8].copy_from_slice(&hash);
    image[0x300..0x400].copy_from_slice(&spl.n().to_bytes_be());
    for (i, pem) in [
        include_str!("../keys/uboot_pubkey_pub.key"),
        include_str!("../keys/kernel_pubkey_pub.key"),
        include_str!("../keys/rootfs_pubkey_pub.key"),
    ]
    .iter()
    .enumerate()
    {
        let key = RsaPublicKey::from_public_key_pem(pem)?;
        let offset = 0x400 + i * 256;
        image[offset..offset + 256].copy_from_slice(&key.n().to_bytes_be());
    }
    let signature = root.sign(
        Pkcs1v15Sign::new::<Sha256>(),
        &Sha256::digest(&image[0x100..0xb00]),
    )?;
    image[0xb00..0xc00].copy_from_slice(&signature);
    header(&mut image[0xfe0..0x1000], size);
    image[0x1000..0x1000 + raw.len()].copy_from_slice(raw);
    let signature = spl.sign(
        Pkcs1v15Sign::new::<Sha256>(),
        &Sha256::digest(&image[0xfe0..]),
    )?;
    image.extend(signature);
    Ok(image)
}

fn header(header: &mut [u8], size: usize) {
    header[..4].copy_from_slice(b"AIHD");
    header[4] = 1;
    header[8..16].copy_from_slice(&(size as u64).to_le_bytes());
    header[24..32].fill(0xa5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_and_signatures() {
        let raw: Vec<_> = (0..35).collect();
        let image = wrap(&raw).unwrap();
        // Archived vendor-tool output for bytes 0..35 and the same development keys.
        assert_eq!(
            format!("{:x}", Sha256::digest(&image)),
            "4b2b3241138204ec0ec61ae35ac2e1681c7765cb400e594a69ff521e6fb37f99"
        );
        assert_eq!(image.len(), 0x1140);
        assert_eq!(&image[0x100..0x108], b"AIHD\x01\0\0\0");
        assert_eq!(&image[0xfe8..0xff0], &64u64.to_le_bytes());
        assert_eq!(&image[0x1000..0x1023], raw);
        assert_eq!(&image[0x1023..0x1040], &[0; 29]);
        for (pem, data, signature) in [
            (
                include_str!("../keys/rsakeypair0_prv.key"),
                &image[0x100..0xb00],
                &image[0xb00..0xc00],
            ),
            (
                include_str!("../keys/spl_pubkey_prv.key"),
                &image[0xfe0..0x1040],
                &image[0x1040..],
            ),
        ] {
            let key = RsaPrivateKey::from_pkcs1_pem(pem).unwrap().to_public_key();
            key.verify(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(data),
                signature,
            )
            .unwrap();
        }
    }

    #[test]
    fn rejects_empty_and_oversized() {
        assert!(wrap(&[]).is_err());
        assert!(wrap(&vec![0; crate::elf2bin::LIMIT + 1]).is_err());
    }
}
