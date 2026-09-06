//! QEMU virt smoke test for independent hart stacks and shared initialization.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
mod bare {
    use core::{
        arch::{asm, naked_asm},
        panic::PanicInfo,
        sync::atomic::{AtomicUsize, Ordering},
    };
    use spacemit_rt::{halt, start};

    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".text.entry")]
    #[unsafe(naked)]
    unsafe extern "C" fn _start() -> ! {
        // Delay the boot hart by default; override to test a late secondary.
        naked_asm!(
            "csrr    t0, mhartid
            lui     t1, %hi(_delay_hart_id)
            addi    t1, t1, %lo(_delay_hart_id)
            bne     t0, t1, 3f
            li      t1, 2000000
        2:  addi    t1, t1, -1
            bnez    t1, 2b
        3:  tail    {start}",
            start = sym start,
        );
    }

    static mut DATA: [u8; 19] = [0xa5; 19];
    static mut BSS: [u8; 21] = [0; 21];
    static ARRIVED: AtomicUsize = AtomicUsize::new(0);

    #[unsafe(no_mangle)]
    extern "C" fn __spacemit_rt_main() -> ! {
        let (hart, thread_pointer, sp, top, size, max_hart): (
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        );
        // SAFETY: this fixture runs in M-mode; these instructions only read
        // registers and linker constants, without accessing memory or the stack.
        unsafe {
            asm!(
                "csrr    {hart}, mhartid
                mv      {thread_pointer}, tp
                mv      {sp}, sp
                lla     {top}, _stack_start
                lui     {size}, %hi(_hart_stack_size)
                addi    {size}, {size}, %lo(_hart_stack_size)
                lui     {max_hart}, %hi(_max_hart_id)
                addi    {max_hart}, {max_hart}, %lo(_max_hart_id)",
                hart = out(reg) hart,
                thread_pointer = out(reg) thread_pointer,
                sp = out(reg) sp,
                top = out(reg) top,
                size = out(reg) size,
                max_hart = out(reg) max_hart,
                options(nostack, nomem),
            );
        }
        assert!(hart <= max_hart && max_hart < usize::BITS as usize - 1);
        assert_eq!(thread_pointer, hart);
        assert_eq!(sp % 16, 0);
        let mie: usize;
        // SAFETY: this fixture runs in M-mode and only reads the local CSR.
        unsafe {
            asm!("csrr {0}, mie", out(reg) mie, options(nostack, nomem));
        }
        assert_eq!(mie, 0, "startup left an interrupt enabled");
        let hart_top = top - hart * size;
        assert!(sp > hart_top - size && sp <= hart_top);
        // SAFETY: startup has finished writing these shared arrays. Every hart
        // only reads them; the acquire barrier must make initialization visible.
        unsafe {
            assert_eq!(core::ptr::addr_of!(DATA).read_volatile(), [0xa5; 19]);
            assert_eq!(core::ptr::addr_of!(BSS).read_volatile(), [0; 21]);
        }
        let bit = 1usize << hart;
        let previous = ARRIVED.fetch_or(bit, Ordering::AcqRel);
        assert_eq!(previous & bit, 0, "hart entered Rust twice");
        let expected = (1usize << (max_hart + 1)) - 1;
        if previous | bit == expected {
            finish(true);
        }
        // SAFETY: this hart has finished; the last arriving hart exits QEMU.
        unsafe { halt() }
    }

    fn finish(passed: bool) -> ! {
        // SAFETY: this test runs only on QEMU virt, whose test finisher is here.
        unsafe { (0x10_0000 as *mut u32).write_volatile(if passed { 0x5555 } else { 0x3333 }) };
        // SAFETY: no test work remains if the finisher does not terminate QEMU.
        unsafe { halt() }
    }

    #[panic_handler]
    fn panic(_: &PanicInfo<'_>) -> ! {
        finish(false)
    }
}
