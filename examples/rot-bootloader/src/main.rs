#![no_std]
#![no_main]

use core::arch::asm;
use spacemit_hal::{prelude::*, uart::Config};
use spacemit_rt::{Peripherals, entry};

// K1/M1 BootROM loads the complete image into SRAM and configures UART0.
// MUSE-Card's vendor DTS selects pinctrl_uart0_2: GPIO68/F2 TX, GPIO69/F2 RX.
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/arch/riscv/boot/dts/spacemit/k1-x_MUSE-Card.dts
#[entry]
fn main() {
    // SAFETY: MMIO and BootROM's power/clocks remain valid; no conflicting users or DMA.
    let Some(mut p) = (unsafe { Peripherals::take() }) else {
        return;
    };
    let Ok(mut uart) = p.uart0.blocking(
        (p.gpio.gpio68, p.gpio.gpio69),
        &mut p.apbc_clocks.uart0,
        Config::default(),
    ) else {
        return;
    };
    writeln!(uart, "Hello world!").unwrap();
    uart.flush();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        // SAFETY: this M-mode hart may wait indefinitely.
        unsafe { asm!("wfi", options(nomem, nostack)) };
    }
}
