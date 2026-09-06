use super::{Function, Input, Output};
use embedded_hal::digital::PinState;

/// Converts an owned or mutably borrowed pad into a configured GPIO mode.
pub trait PadExt<'a> {
    /// Selects GPIO input mode.
    fn into_input(self) -> Input<'a>;

    /// Selects GPIO output mode with the specified initial level.
    fn into_output(self, initial_state: PinState) -> Output<'a>;

    /// Selects the pad's three-bit alternate-function number.
    fn into_function<const F: u8>(self) -> Function<'a, F>;
}
