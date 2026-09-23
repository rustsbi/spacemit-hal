//! Byte-wide 16550A registers on the V100 ACPI platform (`RSCV0003`).
//!
//! Unlike K1/K3, the register stride is one byte and there is no XScale UUE.
//! DLAB selects the divisor aliases at offsets 0 and 1. These raw registers
//! deliberately do not implement the K1/K3 clocked UART interface.

use crate::register::{RC, RWNoModify};
use volatile_register::RW;

/// Eight byte-wide UART registers, with destructive and multiplexed accesses explicit.
#[repr(C)]
pub struct RegisterBlock {
    /// Receive FIFO (read), transmit FIFO (write), or divisor low byte (DLAB=1).
    pub data_divisor_low: RWNoModify<u8>,
    /// Interrupt enables, or divisor high byte when DLAB=1.
    pub interrupt_enable_divisor_high: RW<u8>,
    /// Interrupt identification (read) or FIFO control/reset (write).
    pub interrupt_identification_fifo_control: RWNoModify<u8>,
    /// Line format and divisor-latch access bit.
    pub line_control: RW<u8>,
    /// Modem control.
    pub modem_control: RW<u8>,
    /// Line status; reads may clear error indications.
    pub line_status: RC<u8>,
    /// Modem status; reads clear delta indications.
    pub modem_status: RC<u8>,
    /// Scratch byte.
    pub scratch: RW<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn byte_wide_layout() {
        assert_eq!(offset_of!(RegisterBlock, data_divisor_low), 0);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable_divisor_high), 1);
        assert_eq!(
            offset_of!(RegisterBlock, interrupt_identification_fifo_control),
            2
        );
        assert_eq!(offset_of!(RegisterBlock, line_control), 3);
        assert_eq!(offset_of!(RegisterBlock, modem_control), 4);
        assert_eq!(offset_of!(RegisterBlock, line_status), 5);
        assert_eq!(offset_of!(RegisterBlock, modem_status), 6);
        assert_eq!(offset_of!(RegisterBlock, scratch), 7);
        assert_eq!(size_of::<RegisterBlock>(), 8);
        assert_eq!(align_of::<RegisterBlock>(), 1);
    }
}
