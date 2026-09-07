//! CRC-checked K1 NOR environment and partition locations.

pub const ENV_OFFSET: u32 = 0x60000;
pub const ENV_SIZE: usize = 0x4000;
pub const ENV_MAX_SIZE: usize = 0x10000;

/// Invalid NOR metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Integrity,
    Format,
    Missing,
    Bounds,
    Overlap,
    Duplicate,
}

/// A bounded NOR partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Partition {
    pub offset: u32,
    pub size: u32,
}

/// Boot partitions selected from the NOR environment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub env_size: usize,
    pub fsbl: Partition,
    pub sbi: Partition,
    pub payload: Partition,
}

impl Layout {
    /// Validates the environment and resolves its named partitions.
    pub fn from_env(env: &[u8], capacity: u32) -> Result<Self, Error> {
        if !matches!(env.len(), ENV_SIZE | ENV_MAX_SIZE) || capacity < ENV_OFFSET + env.len() as u32
        {
            return Err(Error::Bounds);
        }
        let expected = u32::from_le_bytes(env[..4].try_into().unwrap());
        let env_size = if crc32(&env[4..ENV_SIZE]) == expected {
            ENV_SIZE
        } else if env.len() == ENV_MAX_SIZE && crc32(&env[4..]) == expected {
            ENV_MAX_SIZE
        } else {
            return Err(Error::Integrity);
        };
        let mut parts = None;
        let mut terminated = false;
        for item in env[4..env_size].split(|&byte| byte == 0) {
            if item.is_empty() {
                terminated = true;
                break;
            }
            if let Some(value) = item.strip_prefix(b"mtdparts=")
                && parts.replace(value).is_some()
            {
                return Err(Error::Duplicate);
            }
        }
        if !terminated {
            return Err(Error::Format);
        }
        let parts = parts.ok_or(Error::Missing)?;
        if parts.len() > 1024 {
            return Err(Error::Bounds);
        }
        let parts = core::str::from_utf8(parts).map_err(|_| Error::Format)?;
        let mut device = None;
        for definition in parts.strip_prefix("mtdparts=").unwrap_or(parts).split(';') {
            let (name, partitions) = definition.split_once(':').ok_or(Error::Format)?;
            if name == "d420c000.spi-0" && device.replace(partitions).is_some() {
                return Err(Error::Duplicate);
            }
        }
        Self::partitions(device.ok_or(Error::Missing)?, capacity, env_size)
    }

    fn partitions(text: &str, capacity: u32, env_size: usize) -> Result<Self, Error> {
        let mut entries = [None; 16];
        let mut end = 0;
        let mut remaining = false;
        for (index, item) in text.split(',').enumerate() {
            if index == entries.len() || remaining {
                return Err(Error::Bounds);
            }
            let (range, name) = item.split_once('(').ok_or(Error::Format)?;
            let (name, flags) = name.split_once(')').ok_or(Error::Format)?;
            if name.is_empty() || !matches!(flags, "" | "ro" | "lk" | "rolk") {
                return Err(Error::Format);
            }
            if entries[..index]
                .iter()
                .flatten()
                .any(|(previous, _)| *previous == name)
            {
                return Err(Error::Duplicate);
            }
            let (size, offset) = match range.split_once('@') {
                Some((size, offset)) => (size, number(offset)?),
                None => (range, end),
            };
            remaining = size == "-";
            let size = if remaining {
                capacity.checked_sub(offset).ok_or(Error::Bounds)?
            } else {
                number(size)?
            };
            if offset < end {
                return Err(Error::Overlap);
            }
            end = offset.checked_add(size).ok_or(Error::Bounds)?;
            if size == 0 || end > capacity {
                return Err(Error::Bounds);
            }
            entries[index] = Some((name, Partition { offset, size }));
        }
        let find = |name| {
            entries
                .iter()
                .flatten()
                .find(|(n, _)| *n == name)
                .map(|(_, partition)| *partition)
                .ok_or(Error::Missing)
        };
        let env = find("env")?;
        let fsbl = find("fsbl")?;
        let sbi = find("opensbi")?;
        let payload = find("uboot")?;
        if env.offset != ENV_OFFSET
            || env.size < env_size as u32
            || fsbl.offset != 0x20000
            || fsbl.size != 0x40000
            || sbi.offset < ENV_OFFSET + env.size
            || payload.offset < ENV_OFFSET + env.size
        {
            return Err(Error::Bounds);
        }
        Ok(Self {
            env_size,
            fsbl,
            sbi,
            payload,
        })
    }
}

fn number(text: &str) -> Result<u32, Error> {
    let (digits, scale) = match text.as_bytes().last() {
        Some(b'k' | b'K') => (&text[..text.len() - 1], 1024),
        Some(b'm' | b'M') => (&text[..text.len() - 1], 1024 * 1024),
        Some(b'g' | b'G') => (&text[..text.len() - 1], 1024 * 1024 * 1024),
        _ => (text, 1),
    };
    let (digits, radix) = digits
        .strip_prefix("0x")
        .map_or((digits, 10), |digits| (digits, 16));
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(Error::Format);
    }
    u32::from_str_radix(digits, radix)
        .map_err(|_| Error::Bounds)?
        .checked_mul(scale)
        .ok_or(Error::Bounds)
}

/// Computes the vendor environment and FIT CRC32.
pub fn crc32(bytes: &[u8]) -> u32 {
    !crc32_update(u32::MAX, bytes)
}

pub(crate) fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    const TABLE: [u32; 16] = [
        0, 0x1db71064, 0x3b6e20c8, 0x26d930ac, 0x76dc4190, 0x6b6b51f4, 0x4db26158, 0x5005713c,
        0xedb88320, 0xf00f9344, 0xd6d6a3e8, 0xcb61b38c, 0x9b64c2b0, 0x86d3d2d4, 0xa00ae278,
        0xbdbdf21c,
    ];
    for &byte in bytes {
        crc ^= u32::from(byte);
        crc = (crc >> 4) ^ TABLE[(crc & 15) as usize];
        crc = (crc >> 4) ^ TABLE[(crc & 15) as usize];
    }
    crc
}
