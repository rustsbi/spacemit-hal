//! Allocation-free NOR reads over a controller-independent QSPI backend.

#![no_std]
#![deny(missing_docs)]

pub mod backend;
mod error;
mod sfdp;

use backend::{AddressSize, Backend, Transfer};
pub use embedded_storage::nor_flash::ReadNorFlash;
pub use error::Error;

/// Flash read protocol and capacity supplied by a board or discovered through SFDP.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parameters {
    /// Capacity in bytes.
    pub capacity: u32,
    /// Memory-read template whose address value is replaced for each chunk.
    pub read: Transfer,
}

impl Parameters {
    /// Creates a conventional 03h, three-byte-address read protocol.
    pub const fn new(capacity: u32) -> Self {
        Self {
            capacity,
            read: Transfer::new(0x03).with_address(0, AddressSize::Three),
        }
    }
}

/// Bounded status polling configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// Maximum 05h status reads per readiness check, not a duration.
    pub poll_budget: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_budget: 10_000,
        }
    }
}

/// An exclusively owned NOR backend with a fixed read protocol.
pub struct NorFlash<B> {
    backend: B,
    parameters: Parameters,
    config: Config,
}

impl<B: Backend> NorFlash<B> {
    /// Probes an already single-line, three-byte-address flash up to 16 MiB using SFDP.
    pub fn probe(mut backend: B, config: Config) -> Result<Self, Error<B::Error>> {
        validate_backend::<B>(config)?;
        wait_ready(&mut backend, config.poll_budget)?;
        let id = read_id(&mut backend)?;
        if matches!(id[0], 0 | 0x7f | 0xff) || matches!(id[2], 0 | 0xff) {
            return Err(Error::InvalidId);
        }
        let parameters = sfdp::discover(&mut backend)?;
        Self::new(backend, parameters, config)
    }

    /// Uses explicit parameters for an already configured flash in the selected read mode.
    pub fn new(
        backend: B,
        parameters: Parameters,
        config: Config,
    ) -> Result<Self, Error<B::Error>> {
        validate_backend::<B>(config)?;
        let address = parameters.read.address.ok_or(Error::InvalidConfig)?;
        if parameters.read.instruction_width != backend::Width::Single {
            return Err(Error::InvalidConfig);
        }
        let address_limit = 1u64 << (8 * address.size as u8);
        if parameters.capacity == 0 || u64::from(parameters.capacity) > address_limit {
            return Err(Error::OutOfBounds);
        }
        Ok(Self {
            backend,
            parameters,
            config,
        })
    }

    /// Reads the three-byte JEDEC identification in single-line SPI mode.
    pub fn jedec_id(&mut self) -> Result<[u8; 3], Error<B::Error>> {
        read_id(&mut self.backend)
    }

    /// Reads the SFDP address space using 5Ah with three address bytes and eight dummy clocks.
    pub fn read_sfdp(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error<B::Error>> {
        sfdp::read(&mut self.backend, offset, bytes)
    }

    /// Returns the selected read protocol and capacity.
    pub const fn parameters(&self) -> Parameters {
        self.parameters
    }
}

impl<B: Backend> embedded_storage::nor_flash::ErrorType for NorFlash<B> {
    type Error = Error<B::Error>;
}

impl<B: Backend> ReadNorFlash for NorFlash<B> {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        // Subtraction avoids overflow on both RV32 and RV64.
        if offset > self.parameters.capacity
            || bytes.len() > (self.parameters.capacity - offset) as usize
        {
            return Err(Error::OutOfBounds);
        }
        if bytes.is_empty() {
            return Ok(());
        }
        wait_ready(&mut self.backend, self.config.poll_budget)?;
        read_chunks(&mut self.backend, self.parameters.read, offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.parameters.capacity as usize
    }
}

fn validate_backend<B: Backend>(config: Config) -> Result<(), Error<B::Error>> {
    if B::MAX_READ_SIZE == 0 || config.poll_budget == 0 {
        return Err(Error::InvalidConfig);
    }
    Ok(())
}

fn read_id<B: Backend>(backend: &mut B) -> Result<[u8; 3], Error<B::Error>> {
    // Identification is not an addressable stream and cannot be split.
    if B::MAX_READ_SIZE < 3 {
        return Err(Error::InvalidConfig);
    }
    let mut id = [0; 3];
    backend
        .read(Transfer::new(0x9f), &mut id)
        .map_err(Error::Backend)?;
    Ok(id)
}

fn wait_ready<B: Backend>(backend: &mut B, budget: u32) -> Result<(), Error<B::Error>> {
    for _ in 0..budget {
        let mut status = [0];
        backend
            .read(Transfer::new(0x05), &mut status)
            .map_err(Error::Backend)?;
        if status[0] & 1 == 0 {
            return Ok(());
        }
    }
    Err(Error::PollLimit)
}

fn read_chunks<B: Backend>(
    backend: &mut B,
    mut transfer: Transfer,
    mut offset: u32,
    bytes: &mut [u8],
) -> Result<(), Error<B::Error>> {
    for chunk in bytes.chunks_mut(B::MAX_READ_SIZE) {
        transfer.address.as_mut().ok_or(Error::InvalidConfig)?.value = offset;
        backend.read(transfer, chunk).map_err(Error::Backend)?;
        // All callers validate their complete range before the first transaction.
        offset += chunk.len() as u32;
    }
    Ok(())
}
