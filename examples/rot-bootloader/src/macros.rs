//! Board console output macros.

/// Prints formatted text without a newline on the board console.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(::core::format_args!($($arg)*))
    };
}

/// Prints a formatted line on the board console.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", ::core::format_args!($($arg)*))
    };
}

/// Prints a diagnostic line through the shared UART console.
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::println!($($arg)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn printing_accepts_captures_and_evaluates_arguments_once() {
        let value = 42;
        let mut calls = 0;
        crate::print!("");
        crate::println!();
        crate::eprintln!();
        crate::print!("{value}: {}", {
            calls += 1;
            calls
        });
        crate::println!("{value}: {}", {
            calls += 1;
            calls
        });
        crate::eprintln!("{value}: {}", {
            calls += 1;
            calls
        });
        assert_eq!(calls, 3);
    }
}
