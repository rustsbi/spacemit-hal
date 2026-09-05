//! K1/M1 generic counter control register.
use super::Control;
use volatile_register::RW;

// Only the control word used by vendor spl.c is modeled; no K3 layout is assumed.

/// K1/M1 generic counter control registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Counter enable control.
    pub control: RW<Control>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0);
        assert_eq!(size_of::<RegisterBlock>(), 4);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
