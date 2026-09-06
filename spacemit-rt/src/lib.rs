//! Bare-metal runtime support for SpacemiT K1 and K3 SoCs.

#![no_std]
#![deny(missing_docs)]

#[macro_use]
mod macros;

pub use spacemit_rt_macros::entry;

// cfg_if selects the first match, so reject conflicting choices independently.
const _: () = assert!(
    cfg!(feature = "nuclei-n308") as u8
        + cfg!(feature = "spacemit-a100") as u8
        + cfg!(feature = "spacemit-rt24") as u8
        + cfg!(feature = "spacemit-x100") as u8
        + cfg!(feature = "spacemit-x60") as u8
        <= 1,
    "select at most one spacemit-rt core/cluster feature"
);

pub mod arch;
pub mod soc;

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "none", feature = "nuclei-n308"))] {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "riscv32")] {
                pub use arch::nuclei_n308::{halt, start};
            } else {
                compile_error!("nuclei-n308 / k1-mcu requires a riscv32 bare-metal target");
            }
        }
    } else if #[cfg(all(target_os = "none", any(
        feature = "spacemit-a100", feature = "spacemit-rt24",
        feature = "spacemit-x60", feature = "spacemit-x100"
    )))] {
        cfg_if::cfg_if! {
            if #[cfg(not(target_arch = "riscv64"))] {
                compile_error!("SpacemiT X60/X100/A100/RT24 requires a riscv64 bare-metal target");
            } else if #[cfg(feature = "spacemit-a100")] {
                pub use arch::spacemit_a100::{halt, start};
            } else if #[cfg(feature = "spacemit-rt24")] {
                pub use arch::spacemit_rt24::{halt, start};
            } else if #[cfg(feature = "spacemit-x60")] {
                pub use arch::spacemit_x60::{halt, start};
            } else if #[cfg(feature = "spacemit-x100")] {
                pub use arch::spacemit_x100::{halt, start};
            }
        }
    } else {
        /// Re-exported dummy start.
        #[doc(hidden)]
        pub fn start() -> ! { unimplemented!() }
        /// Re-exported dummy halt.
        #[doc(hidden)]
        pub fn halt() -> ! { unimplemented!() }
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "k1-cpu", feature = "k1-mcu"))] {
        pub use soc::k1::Peripherals;
    } else if #[cfg(any(feature = "k3-cpu", feature = "k3-ai", feature = "k3-mcu"))] {
        pub use soc::k3::Peripherals;
    }
}
