use sbi_rt::legacy::*;
use core::fmt::{Write, Result as FmtResult, Error as FmtError, Arguments};

pub struct Stdin;

impl Stdin {
    #[allow(deprecated)]
    pub fn read_byte() -> usize {
        console_getchar()
    }
}

pub struct Stdout;

impl Stdout {
    #[allow(deprecated)]
    pub fn write_byte(byte: u8) -> FmtResult {
        let byte = byte as usize;
        
        if console_putchar(byte) == byte {
            Ok(())
        } else {
            Err(FmtError)
        }
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> FmtResult {
        for c in s.bytes() {
            Self::write_byte(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> FmtResult {
        Self::write_byte(c as u8)
    }
}

/// Logs a message to kernel output console.
/// Formatting is supported.
#[macro_export]
macro_rules! print {
    () => {};
    
    ($($arg: tt)*) => {
        $crate::sbi::console::_print(format_args!($($arg)*));
    }
}

/// Logs a message to kernel output console and starts a new line.
/// Formatting is supported.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };

    ($($arg: tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate warnings.
#[macro_export]
macro_rules! warn_print {
    ($($arg: tt)*) => {
        $crate::print!("\x1b[093m{}\x1b[037m", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate warnings, and start a new line.
#[macro_export]
macro_rules! warn {
    ($($arg: tt)*) => {
        $crate::warn_print!("{}\n", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate errors.
#[macro_export]
macro_rules! error_print {
    ($($arg: tt)*) => {
        $crate::print!("\x1b[091m{}\x1b[037m", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate errors, and start a new line.
#[macro_export]
macro_rules! error {
    ($($arg: tt)*) => {
        $crate::error_print!("{}\n", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate informations.
#[macro_export]
macro_rules! info_print {
    ($($arg: tt)*) => {
        $crate::print!("\x1b[036m{}\x1b[037m", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate informations, and start a new line.
#[macro_export]
macro_rules! info {
    ($($arg: tt)*) => {
        $crate::info_print!("{}\n", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate debug informations.
#[macro_export]
macro_rules! debug_print {
    ($($arg: tt)*) => {
        $crate::print!("\x1b[032m{}\x1b[037m", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate debug informations, and start a new line.
#[macro_export]
macro_rules! debug {
    ($($arg: tt)*) => {
        $crate::debug_print!("{}\n", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate stack tracing informations.
#[macro_export]
macro_rules! trace_print {
    ($($arg: tt)*) => {
        $crate::print!("\x1b[090m{}\x1b[037m", format_args!($($arg)*));
    }
}

/// Logs a message to console in yellow to indicate stack tracing informations, and start a new line.
#[macro_export]
macro_rules! trace {
    ($($arg: tt)*) => {
        $crate::trace_print!("{}\n", format_args!($($arg)*));
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}
