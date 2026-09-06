#![no_std]
#![no_main]

use core::arch::asm;
use embedded_io::Write;
use spacemit_hal::uart::{BlockingUart, Config};
use spacemit_rt::{Peripherals, entry};

// K1/M1 BootROM loads the complete image into SRAM and configures UART0.
#[entry]
fn main() {
    // SAFETY: K1/M1 MMIO is identity-mapped; take grants access to only one hart.
    let Some(mut p) = (unsafe { Peripherals::take() }) else {
        return;
    };
    // SAFETY: BootROM left UART0 idle with live clocks, pads, and divisor;
    // no other hart, interrupt handler, or DMA engine accesses it, and this
    // platform retains BootROM's upstream clock configuration.
    let Ok(mut uart) =
        (unsafe { BlockingUart::new(&mut p.uart0, &mut p.apbc_clocks.uart0, Config::default()) })
    else {
        return;
    };
    writeln!(uart, "hello world\r").unwrap();
    uart.flush();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        // SAFETY: this M-mode hart may wait indefinitely.
        unsafe { asm!("wfi", options(nomem, nostack)) };
    }
}
