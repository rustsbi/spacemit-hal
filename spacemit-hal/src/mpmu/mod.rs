//! Main power-management clock controls.
mod register;
pub use register::{ApplicationClockGate, k1, k3};

/// An exclusive MPMU token or its mutable borrow.
///
/// # Safety
/// Transfer one SoC's MPMU registers with exclusive, valid access for `'a`.
/// Power and upstream sources must remain stable. Untracked consumers must
/// tolerate shared-gate changes; tracked consumers must retain the controller
/// borrow, including after dropping `Clocks`.
pub unsafe trait Instance<'a> {
    /// The register layout belonging to this SoC.
    type RegisterBlock: 'a;
    /// Consumes the token or borrow, retaining exclusive access for `'a`.
    fn register_block(self) -> &'a Self::RegisterBlock;
}
