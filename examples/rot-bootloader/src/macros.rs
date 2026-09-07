//! Board console output macros.

/// Prints formatted text without a newline on the board console.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::__ufmt as ufmt;
        $crate::console::_print(|writer| ufmt::uwrite!(writer, $($arg)*))
    }};
}

/// Prints a formatted line on the board console.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        use $crate::__ufmt as ufmt;
        $crate::console::_print(|writer| ufmt::uwriteln!(writer, $($arg)*))
    }};
}

/// Prints a cold-path diagnostic using `core::fmt`, including external errors.
#[macro_export]
macro_rules! eprintln {
    () => { $crate::console::_eprint(::core::format_args!("\n")) };
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::console::_eprint(::core::format_args!(concat!($fmt, "\n") $(, $arg)*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn printing_evaluates_arguments_once_even_without_a_console() {
        let value = 42;
        let mut calls = 0;
        crate::print!("");
        crate::println!();
        crate::eprintln!();
        crate::print!("{}: {}", value, {
            calls += 1;
            calls
        });
        crate::println!("{}: {}", value, {
            calls += 1;
            calls
        });
        crate::eprintln!("{}: {}", value, {
            calls += 1;
            calls
        });
        assert_eq!(calls, 3);
    }
}
