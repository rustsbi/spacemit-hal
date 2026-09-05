//! I2C control and byte-buffer access.

use vcell::VolatileCell;

/// I2C configuration and commands with explicit full-word writes.
#[repr(transparent)]
pub struct ControlRegister {
    register: VolatileCell<u32>,
}

impl ControlRegister {
    /// Reads the current configuration and command state.
    #[inline]
    pub fn read(&self) -> u32 {
        self.register.get()
    }

    /// Writes a complete configuration or command value.
    ///
    /// # Safety
    /// The caller must exclusively own the accessible controller and ensure
    /// every selected configuration, reset, and transfer command is valid for
    /// its current bus state; pending command bits must not be replayed from a read.
    #[inline]
    pub unsafe fn write(&self, value: u32) {
        self.register.set(value);
    }
}

/// I2C byte buffer accessed through 32-bit bus transactions.
#[repr(transparent)]
pub struct DataBuffer {
    register: VolatileCell<u32>,
}

impl DataBuffer {
    /// Reads the received byte.
    #[inline]
    pub fn read(&self) -> u8 {
        self.register.get() as u8
    }

    /// Loads the next transmitted byte.
    ///
    /// # Safety
    /// The caller must exclusively own the accessible controller and ensure
    /// the buffer is ready and no transfer, interrupt handler, or DMA user can
    /// consume or overwrite it before the intended command.
    #[inline]
    pub unsafe fn write(&self, byte: u8) {
        self.register.set(u32::from(byte));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn register_access_layout() {
        assert_eq!(size_of::<ControlRegister>(), 4);
        assert_eq!(align_of::<ControlRegister>(), 4);
        assert_eq!(size_of::<DataBuffer>(), 4);
        assert_eq!(align_of::<DataBuffer>(), 4);
    }

    #[test]
    fn control_writes_do_not_merge_old_commands() {
        let control = ControlRegister {
            register: VolatileCell::new(u32::MAX),
        };
        for value in [1 << 10, 0x0020_6000, 0x0020_6009, 0] {
            // SAFETY: This is exclusively owned mock memory, not hardware.
            unsafe { control.write(value) };
            assert_eq!(control.read(), value);
        }
    }

    #[test]
    fn data_access_uses_a_word_and_exposes_only_the_byte() {
        let data = DataBuffer {
            register: VolatileCell::new(0xabcd_125a),
        };
        assert_eq!(data.read(), 0x5a);
        for byte in [0, 1, 0x80, 0xff] {
            // SAFETY: This is exclusively owned mock memory, not hardware.
            unsafe { data.write(byte) };
            assert_eq!(data.register.get(), u32::from(byte));
            assert_eq!(data.read(), byte);
        }
    }
}
