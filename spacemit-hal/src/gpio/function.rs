use super::FlexPad;

/// A pad configured for alternate function F.
#[must_use]
pub struct Function<'a, const F: u8> {
    pub(super) pad: FlexPad<'a>,
}

impl<'a, const F: u8> From<Function<'a, F>> for FlexPad<'a> {
    #[inline]
    fn from(function: Function<'a, F>) -> Self {
        function.pad
    }
}
