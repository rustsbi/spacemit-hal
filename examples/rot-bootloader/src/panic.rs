#[cfg(target_os = "none")]
use core::{hint::spin_loop, panic::PanicInfo};

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    crate::eprintln!("{info}");
    loop {
        spin_loop();
    }
}
