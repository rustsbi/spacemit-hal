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
