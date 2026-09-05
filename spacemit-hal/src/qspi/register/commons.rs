//! QSPI register access with explicit FIFO-clear commands.

use vcell::VolatileCell;

/// Module control with separate configuration and self-clearing command access.
#[repr(transparent)]
pub struct ModuleControl {
    register: VolatileCell<u32>,
}

impl ModuleControl {
    const CLEAR_RX_FIFO: u32 = 1 << 10;
    const CLEAR_TX_FIFO: u32 = 1 << 11;
    const COMMANDS: u32 = Self::CLEAR_RX_FIFO | Self::CLEAR_TX_FIFO;

    /// Reads module configuration without replayable FIFO-clear commands.
    pub fn read_configuration(&self) -> u32 {
        self.register.get() & !Self::COMMANDS
    }

    /// Writes module configuration, rejecting FIFO-clear command bits.
    ///
    /// # Safety
    /// The caller must exclusively own the controller and ensure the value,
    /// clocks, reset sequencing, and absence of active IP/AHB/DMA users permit
    /// this configuration change.
    pub unsafe fn write_configuration(&self, value: u32) {
        assert_eq!(
            value & Self::COMMANDS,
            0,
            "use clear_fifos for FIFO commands"
        );
        self.register.set(value);
    }

    /// Clears both FIFOs while preserving module configuration.
    ///
    /// # Safety
    /// The caller must exclusively own an idle, accessible controller with no
    /// IP/AHB/DMA users and no pending FIFO data that must be preserved.
    pub unsafe fn clear_fifos(&self) {
        self.register
            .set(self.read_configuration() | Self::COMMANDS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn fifo_commands_are_not_replayed_as_configuration() {
        assert_eq!(size_of::<ModuleControl>(), 4);
        assert_eq!(align_of::<ModuleControl>(), 4);
        let register = ModuleControl {
            register: VolatileCell::new(0x000f_000c),
        };
        // SAFETY: This is exclusively owned mock memory, not hardware.
        unsafe { register.clear_fifos() };
        assert_eq!(register.register.get(), 0x000f_0c0c);
        assert_eq!(register.read_configuration(), 0x000f_000c);
        // SAFETY: The mock configuration contains no command bits.
        unsafe { register.write_configuration(register.read_configuration() | (1 << 14)) };
        assert_eq!(register.register.get(), 0x000f_400c);
    }

    #[test]
    #[should_panic(expected = "use clear_fifos for FIFO commands")]
    fn rejects_commands_in_configuration() {
        let register = ModuleControl {
            register: VolatileCell::new(0),
        };
        // SAFETY: Owned mock memory; invalid command bits are rejected before writing.
        unsafe { register.write_configuration(1 << 10) };
    }
}
