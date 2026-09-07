//! Boot-hart console. Uninitialized, reentrant and other-hart writes are skipped.

use core::fmt;
use embedded_io::Write;
use spacemit_hal::uart::BlockingUart;
use spin::{Mutex, Once};

struct Console<W> {
    owner: usize,
    writer: Mutex<W>,
}

// SAFETY: Moving this private wrapper does not access the UART; lock still
// requires its original hart. Dropping BlockingUart performs no hardware access.
unsafe impl Send for Console<BlockingUart<'static>> {}
// SAFETY: Only the owner hart can acquire the non-Send UART guard; try_lock
// excludes overlapping interrupt/formatting access.
unsafe impl Sync for Console<BlockingUart<'static>> {}

static CONSOLE: Once<Console<BlockingUart<'static>>> = Once::new();

pub(crate) type Guard<'a, W> = spin::MutexGuard<'a, W>;

impl<W> Console<W> {
    #[inline]
    fn install(slot: &Once<Self>, hart: usize, writer: W) -> Result<(), W> {
        let mut writer = Some(writer);
        slot.call_once(|| Self {
            owner: hart,
            writer: Mutex::new(writer.take().unwrap()),
        });
        writer.map_or(Ok(()), Err)
    }

    #[inline]
    fn lock(&self, hart: usize) -> Option<Guard<'_, W>> {
        if self.owner != hart {
            return None;
        }
        self.writer.try_lock()
    }
}

#[inline]
fn current_hart() -> Option<usize> {
    #[cfg(all(
        target_os = "none",
        any(target_arch = "riscv32", target_arch = "riscv64")
    ))]
    {
        Some(riscv::register::mhartid::read())
    }
    #[cfg(not(all(
        target_os = "none",
        any(target_arch = "riscv32", target_arch = "riscv64")
    )))]
    {
        None
    }
}

pub(crate) fn install(uart: BlockingUart<'static>) -> Result<(), ()> {
    let hart = current_hart().ok_or(())?;
    Console::install(&CONSOLE, hart, uart).map_err(|_| ())
}

#[inline]
pub(crate) fn lock() -> Option<Guard<'static, BlockingUart<'static>>> {
    CONSOLE.get()?.lock(current_hart()?)
}

/// Writes and flushes a formatted message through the board console.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    if let Some(mut uart) = lock() {
        let _ = uart.write_fmt(args);
        uart.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write as _;
    extern crate std;
    use std::string::String;

    // SAFETY: The host mock contains only a hart number and Mutex<String>.
    unsafe impl Send for Console<String> {}
    // SAFETY: All access to the host string is protected by the mutex.
    unsafe impl Sync for Console<String> {}

    #[test]
    fn owns_one_writer_and_rejects_other_harts() {
        let slot = Once::new();
        assert!(slot.get().is_none());
        Console::install(&slot, 0, String::new()).unwrap();
        assert_eq!(
            Console::install(&slot, 1, String::from("replacement")),
            Err(String::from("replacement"))
        );
        let console = slot.get().unwrap();
        assert!(console.lock(1).is_none());
        writeln!(console.lock(0).unwrap(), "value: {}", 42).unwrap();
        assert_eq!(*console.lock(0).unwrap(), "value: 42\n");
    }

    #[test]
    fn reentrant_printing_does_not_alias_or_wait() {
        let slot = Once::new();
        Console::install(&slot, 0, String::new()).unwrap();
        let console = slot.get().unwrap();
        {
            let mut writer = console.lock(0).unwrap();
            assert!(console.lock(0).is_none());
            writer.push_str("outer");
        }
        assert_eq!(*console.lock(0).unwrap(), "outer");
    }

    #[test]
    fn formatting_panic_releases_the_lock() {
        let slot = Once::new();
        Console::install(&slot, 0, String::new()).unwrap();
        let console = slot.get().unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = console.lock(0).unwrap();
            panic!("formatter");
        }));
        assert!(result.is_err());
        assert!(console.lock(0).is_some());
    }

    #[test]
    fn concurrent_install_keeps_one_owner_and_writer() {
        let slot = Once::new();
        std::thread::scope(|scope| {
            let first = scope.spawn(|| Console::install(&slot, 0, String::from("first")));
            let second = scope.spawn(|| Console::install(&slot, 1, String::from("second")));
            assert_ne!(
                first.join().unwrap().is_ok(),
                second.join().unwrap().is_ok()
            );
        });
        let console = slot.get().unwrap();
        assert_eq!(
            *console.lock(console.owner).unwrap(),
            ["first", "second"][console.owner]
        );
        assert!(console.lock(console.owner ^ 1).is_none());
    }

    #[test]
    fn uart_guards_and_stdout_stay_hart_local() {
        trait AmbiguousIfSend<A> {
            fn probe() {}
        }
        impl<T: ?Sized> AmbiguousIfSend<()> for T {}
        impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}
        trait AmbiguousIfSync<A> {
            fn probe() {}
        }
        impl<T: ?Sized> AmbiguousIfSync<()> for T {}
        impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

        let _ = <Guard<'static, BlockingUart<'static>> as AmbiguousIfSend<_>>::probe;
        let _ = <Guard<'static, BlockingUart<'static>> as AmbiguousIfSync<_>>::probe;
        let _ = <crate::io::Stdout as AmbiguousIfSend<_>>::probe;
        let _ = <crate::io::Stdout as AmbiguousIfSync<_>>::probe;
    }
}
