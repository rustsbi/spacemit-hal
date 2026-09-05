use volatile_register::{RO, WO};

/// A borrowed GPIO bank register view.
#[derive(Clone, Copy)]
pub(in crate::gpio) struct BankRegisters<'a> {
    pub pin_level: &'a RO<u32>,
    pub pin_output_set: &'a WO<u32>,
    pub pin_output_clear: &'a WO<u32>,
    pub direction_set: &'a WO<u32>,
    pub direction_clear: &'a WO<u32>,
}

impl<'a> BankRegisters<'a> {
    #[inline(always)]
    pub(in crate::gpio) const fn new(
        pin_level: &'a RO<u32>,
        pin_output_set: &'a WO<u32>,
        pin_output_clear: &'a WO<u32>,
        direction_set: &'a WO<u32>,
        direction_clear: &'a WO<u32>,
    ) -> Self {
        Self {
            pin_level,
            pin_output_set,
            pin_output_clear,
            direction_set,
            direction_clear,
        }
    }
}
