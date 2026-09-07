#[cfg(target_os = "none")]
use core::{hint::spin_loop, panic::PanicInfo};

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        crate::println!(
            "panic at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    if let Some(message) = info.message().as_str() {
        crate::println!("{}", message);
    } else {
        crate::eprintln!("{}", info.message());
    }
    loop {
        spin_loop();
    }
}
