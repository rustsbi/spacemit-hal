//! Bare-metal runtime support for SpacemiT K1 and K3 SoCs.

#![no_std]
#![deny(missing_docs)]

#[macro_use]
mod macros;

pub use spacemit_rt_macros::entry;

// Core features are additive; only cluster/bootrom entry choices conflict.
const _: () = assert!(
    cfg!(feature = "k1-bootrom") as u8
        + cfg!(feature = "k1-cpu") as u8
        + cfg!(feature = "k1-mcu") as u8
        + cfg!(feature = "k3-bootrom") as u8
        + cfg!(feature = "k3-cpu") as u8
        + cfg!(feature = "k3-ai") as u8
        + cfg!(feature = "k3-mcu") as u8
        <= 1,
    "select at most one spacemit-rt cluster/bootrom feature"
);

pub mod arch;
pub mod hart;
pub mod soc;

const _: () = {
    assert!(core::mem::size_of::<soc::k1::Peripherals>() == 0);
    assert!(core::mem::size_of::<soc::k3::Peripherals>() == 0);
};

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "none", feature = "k1-mcu"))] {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "riscv32")] {
                pub use arch::nuclei_n308::{halt, start};
            } else {
                compile_error!("k1-mcu requires a riscv32 bare-metal target");
            }
        }
    } else if #[cfg(all(target_os = "none", any(
        feature = "k1-bootrom", feature = "k1-cpu",
        feature = "k3-bootrom", feature = "k3-cpu", feature = "k3-ai", feature = "k3-mcu"
    )))] {
        cfg_if::cfg_if! {
            if #[cfg(not(target_arch = "riscv64"))] {
                compile_error!("the selected cluster/bootrom feature requires a riscv64 bare-metal target");
            } else if #[cfg(any(feature = "k3-bootrom", feature = "k3-cpu"))] {
                pub use arch::spacemit_x100::{halt, start};
            } else if #[cfg(feature = "k3-ai")] {
                pub use arch::spacemit_a100::{halt, start};
            } else if #[cfg(feature = "k3-mcu")] {
                pub use arch::spacemit_rt24::{halt, start};
            } else if #[cfg(any(feature = "k1-bootrom", feature = "k1-cpu"))] {
                pub use arch::spacemit_x60::{halt, start};
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
    if #[cfg(any(feature = "k1-bootrom", feature = "k1-cpu", feature = "k1-mcu"))] {
        pub use soc::k1::Peripherals;
    } else if #[cfg(any(
        feature = "k3-cpu", feature = "k3-ai", feature = "k3-mcu", feature = "k3-bootrom"
    ))] {
        pub use soc::k3::Peripherals;
    }
}
