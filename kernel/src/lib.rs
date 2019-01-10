#![no_std]
#![feature(abi_x86_interrupt)]

pub mod vga;
pub mod uart;
pub mod init;
pub mod hid;

/// Macro for printing to the standard output, with a newline.
///
/// There is 2 ways of using it: Especifying where to print, or
/// using the standard VGA output in the kernel.
///
/// # Examples
/// ```rust
/// // Especifying
/// kprintln!(VGA, "MyText");
///
/// // Using standard
/// kprintln!("MyText");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => ($crate::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::kprint!(concat!($fmt, "\n"), $($arg)*));
    ($ctx:ident, $fmt:expr) => ($crate::kprint!($ctx, concat!($fmt, "\n")));
    ($ctx:ident, $fmt:expr, $($arg:tt)*) => ($crate::kprint!($ctx, concat!($fmt, "\n"), $($arg)*));
}

/// Macro for printing to the standard output.
///
/// There is 2 ways of using it: Especifying where to print, or
/// using the standard VGA output in the kernel.
///
/// # Examples
/// ```rust
/// // Especifying
/// kprint!(VGA, "MyText");
///
/// // Using standard
/// kprint!("MyText");
/// ```
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::init::vga::VGA.lock().write_fmt(format_args!($($arg)*)).unwrap();
        $crate::init::vga::VGA.lock().flush();
    });
    ($ctx:ident, $($arg:tt)*) => ({
        use core::fmt::Write;
        $ctx.write_fmt(format_args!($($arg)*)).unwrap();
        $ctx.flush();
    });
}
