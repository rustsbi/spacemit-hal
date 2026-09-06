//! SoC-specific peripheral ownership and address maps.

pub mod k1;
pub mod k3;
#[doc(hidden)]
pub mod v100;

// Shared by both SoC modules: overlapping address maps must not yield two owners.
static PERIPHERALS_TAKEN: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

fn claim_peripherals(taken: &core::sync::atomic::AtomicBool) -> bool {
    use core::sync::atomic::Ordering;
    taken
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}

#[cfg(test)]
mod ownership_tests {
    extern crate std;

    #[test]
    fn only_one_hart_claims_peripherals() {
        use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        let taken = AtomicBool::new(false);
        let winners = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..16 {
                scope.spawn(|| {
                    if super::claim_peripherals(&taken) {
                        winners.fetch_add(1, Ordering::Relaxed);
                    }
                });
            }
        });
        assert_eq!(winners.load(Ordering::Relaxed), 1);
        assert!(!super::claim_peripherals(&taken));
    }
}
