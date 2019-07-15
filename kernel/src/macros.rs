/// Macro for printing to the standard output, with a newline.
///
/// There is 2 ways of using it: Especifying where to print, or
/// using the standard VGA output in the kernel.
///
/// # Examples
/// ```rust
/// // Especifying
/// kprintln!(VGA; "MyText");
///
/// // Using standard
/// kprintln!("MyText");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($ctx:ident; $fmt:expr) => ($crate::kprint!($ctx, concat!($fmt, "\n")));
    ($ctx:ident; $fmt:expr, $($arg:tt)*) => ($crate::kprint!($ctx, concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::kprint!(concat!($fmt, "\n"), $($arg)*));
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
    ($ctx:ident; $($arg:tt)*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            $ctx.write_fmt(format_args!($($arg)*)).unwrap();
            $ctx.flush();
        });
    }};
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            $crate::init::vga::VGA.lock().write_fmt(format_args!($($arg)*)).unwrap();
            $crate::init::vga::VGA.lock().flush();
        });
    }};
}

/// Same as `kprintln!`, but using `SERIAL1` as default output
#[macro_export]
macro_rules! s1println {
    ($fmt:expr) => ($crate::s1print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::s1print!(concat!($fmt, "\n"), $($arg)*));
}

/// Same as `kprint!`, but using `SERIAL1` as default output
#[macro_export]
macro_rules! s1print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        $crate::init::serial::SERIAL1.lock().write_fmt(format_args!($($arg)*)).unwrap();
    }};
}

/// Macro to change `VGA` color using the `Color` enum
///
/// # Example
/// ```no_run
/// // Change only foreground
/// vgacolor!(Color::White)
///
/// // Chnage foreground to blue and background to red
/// vgacolor!(Color::Blue, Color::Red)
/// ```
#[macro_export]
macro_rules! vgacolor {
    ($fg:expr) => {{
        $crate::init::vga::VGA.lock().set_foreground($fg);
    }};
    ($fg:expr, $bg:expr) => {{
        $crate::init::vga::VGA.lock().set_foreground($fg);
        $crate::init::vga::VGA.lock().set_background($fg);
    }};
}
