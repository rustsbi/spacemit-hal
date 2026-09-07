//! PLL clock controls.
mod register;
pub use register::{k1, k3};

/// An exclusive APBS token or its mutable borrow.
///
/// # Safety
/// Transfer one SoC's APBS registers with exclusive, valid access for `'a`.
/// Power and PLL sources must remain stable; K1/M1 requires a 24 MHz reference.
/// Untracked consumers must tolerate shared-gate changes; tracked consumers
/// must retain the controller borrow, including after dropping `Clocks`.
pub unsafe trait Instance<'a> {
    /// The register layout belonging to this SoC.
    type RegisterBlock: 'a;
    /// Consumes the token or borrow, retaining exclusive access for `'a`.
    fn register_block(self) -> &'a Self::RegisterBlock;
}
