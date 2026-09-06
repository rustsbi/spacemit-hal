//! Link-only fixture for the feature-selected startup implementation.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
mod bare {
    use core::{arch::naked_asm, panic::PanicInfo};
    use spacemit_rt::{halt, start};

    // Type-check both ends of the argument-free startup interface.
    const _: unsafe extern "C" fn() -> ! = start;
    const _: extern "C" fn() = __spacemit_rt_main;

    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".text.entry")]
    #[unsafe(naked)]
    unsafe extern "C" fn _start() -> ! {
        naked_asm!("tail {start}", start = sym start);
    }

    // Odd lengths exercise section-end padding for XLEN-wide initialization.
    #[unsafe(no_mangle)]
    static mut DATA: [u8; 3] = [0xaa, 0x55, 0xa5];
    #[unsafe(no_mangle)]
    static mut BSS: [u8; 5] = [0; 5];

    #[unsafe(no_mangle)]
    extern "C" fn __spacemit_rt_main() {
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
