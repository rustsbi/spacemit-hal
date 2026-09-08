//! K1/M1 timer and watchdog registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/clocksource/timer-k1x.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/watchdog/spacemit-k1-wdt.c
// The watchdog offsets follow the dedicated watchdog driver, not unused timer macros.

/// K1/M1 timer and watchdog registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Counter enables.
    pub count_enable: RW<u32>,
    /// Counter modes.
    pub count_mode: RW<u32>,
    /// Counter restart commands.
    pub count_restart: WO<u32>,
    /// Counter clock selection.
    pub clock_control: RW<u32>,
    /// Timer 0 match values.
    pub match0: [RW<u32>; 3],
    _padding_0x01c: [u32; 1],
    /// Timer 1 match values.
    pub match1: [RW<u32>; 3],
    _padding_0x02c: [u32; 1],
    /// Timer 2 match values.
    pub match2: [RW<u32>; 3],
    _padding_0x03c: [u32; 1],
    /// Counter preload values.
    pub preload_value: [RW<u32>; 3],
    _padding_0x04c: [u32; 1],
    /// Counter preload selection.
    pub preload_control: [RW<u32>; 3],
    _padding_0x05c: [u32; 1],
    /// Match interrupt enables.
    pub interrupt_enable: [RW<u32>; 3],
    _padding_0x06c: [u32; 1],
    /// Match interrupt acknowledgement commands.
    pub interrupt_clear: [WO<u32>; 3],
    _padding_0x07c: [u32; 1],
    /// Match interrupt status.
    pub status: [RO<u32>; 3],
    _padding_0x08c: [u32; 1],
    /// Synchronized counter values.
    pub count: [RO<u32>; 3],
    _padding_0x09c: [u32; 5],
    /// First watchdog access key (TWFAR).
    pub watchdog_access1: WO<u32>,
    /// Second watchdog access key (TWSAR).
    pub watchdog_access2: WO<u32>,
    /// Watchdog enable and reset selection (TWER).
    pub watchdog_enable: RW<u32>,
    /// Watchdog match count (TWMR).
    pub watchdog_match: RW<u32>,
    /// Watchdog reset status; writing zero clears it (TWSR).
    pub watchdog_status: RWNoModify<u32>,
    /// Watchdog interrupt acknowledgement (TWICLR).
    pub watchdog_interrupt_clear: WO<u32>,
    /// Watchdog counter reset command (TWCR).
    pub watchdog_counter_reset: WO<u32>,
    /// Watchdog count; double-read to verify stability (TWVR).
    pub watchdog_value: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, count_enable), 0x0);
        assert_eq!(offset_of!(RegisterBlock, count_mode), 0x4);
        assert_eq!(offset_of!(RegisterBlock, count_restart), 0x8);
        assert_eq!(offset_of!(RegisterBlock, clock_control), 0xc);
        assert_eq!(offset_of!(RegisterBlock, match0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, match1), 0x20);
        assert_eq!(offset_of!(RegisterBlock, match2), 0x30);
        assert_eq!(offset_of!(RegisterBlock, preload_value), 0x40);
        assert_eq!(offset_of!(RegisterBlock, preload_control), 0x50);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x60);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x70);
        assert_eq!(offset_of!(RegisterBlock, status), 0x80);
        assert_eq!(offset_of!(RegisterBlock, count), 0x90);
        assert_eq!(offset_of!(RegisterBlock, watchdog_access1), 0xb0);
        assert_eq!(offset_of!(RegisterBlock, watchdog_access2), 0xb4);
        assert_eq!(offset_of!(RegisterBlock, watchdog_enable), 0xb8);
        assert_eq!(offset_of!(RegisterBlock, watchdog_match), 0xbc);
        assert_eq!(offset_of!(RegisterBlock, watchdog_status), 0xc0);
        assert_eq!(offset_of!(RegisterBlock, watchdog_interrupt_clear), 0xc4);
        assert_eq!(offset_of!(RegisterBlock, watchdog_counter_reset), 0xc8);
        assert_eq!(offset_of!(RegisterBlock, watchdog_value), 0xcc);
        assert_eq!(size_of::<RegisterBlock>(), 0xd0);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
