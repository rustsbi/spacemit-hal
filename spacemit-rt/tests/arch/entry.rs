//! Link-only fixture for the feature-selected startup implementation.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
mod bare {
    use core::panic::PanicInfo;
    use spacemit_rt::{Peripherals, entry, halt, start};

    // Type-check both ends of the argument-free startup interface.
    const _: unsafe extern "C" fn() -> ! = start;
    const _: unsafe extern "C" fn() = boot;

    // Odd lengths exercise section-end padding for XLEN-wide initialization.
    #[unsafe(no_mangle)]
    static mut DATA: [u8; 3] = [0xaa, 0x55, 0xa5];
    #[unsafe(no_mangle)]
    static mut BSS: [u8; 5] = [0; 5];

    #[entry]
    fn boot(p: Peripherals) {
        core::hint::black_box(p);
        #[cfg(feature = "k3-bootrom")]
        core::hint::black_box((
            spacemit_rt::arch::spacemit_a100::start as unsafe extern "C" fn() -> !,
            spacemit_rt::arch::spacemit_x100::start as unsafe extern "C" fn() -> !,
        ));
        // SAFETY: the startup contract initializes these exclusively owned statics.
        let state = unsafe {
            (
                core::ptr::addr_of!(DATA).read_volatile(),
                core::ptr::addr_of!(BSS).read_volatile(),
            )
        };
        core::hint::black_box(state);
    }

    #[panic_handler]
    fn panic(_: &PanicInfo<'_>) -> ! {
        // SAFETY: all fixture code runs in M-mode.
        unsafe { halt() }
    }
}
