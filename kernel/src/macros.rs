/// Macro for printing to the standard output, with a newline.
///
/// There is 2 ways of using it: Especifying where to print, or
/// using the standard VGA output in the kernel.
///
/// # Examples
/// ```no_run
/// // Especifying
/// // Especifying (Assumes no flush method)
/// kprintln!(SERIAL1; "MyText");
///
/// // Especifying saying that what is passed
/// kprintln!(VGA; flush; "MyText");
///
/// // Using standard
/// kprintln!("MyText");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($ctx:expr; $fmt:expr) => ( $crate::kprint!($ctx; concat!($fmt, "\n")) );
    ($ctx:expr; $fmt:expr, $($arg:tt)*) => ( $crate::kprint!($ctx; concat!($fmt, "\n"), $($arg)*) );
    ($ctx:expr; flush; $fmt:expr) => ( $crate::kprint!($ctx; flush; concat!($fmt, "\n")) );
    ($ctx:expr; flush; $fmt:expr, $($arg:tt)*) => ( $crate::kprint!($ctx; flush; concat!($fmt, "\n"), $($arg)*) );
    ($fmt:expr) => ( $crate::kprint!(concat!($fmt, "\n")) );
    ($fmt:expr, $($arg:tt)*) => ( $crate::kprint!(concat!($fmt, "\n"), $($arg)*) );
}

/// Macro for printing to the standard output.
///
/// There is 3 ways of using it: Especifying where to print, or
/// using the standard VGA output in the kernel.
///
/// # Examples
/// ```no_run
/// // Especifying (Assumes no flush method)
/// kprint!(SERIAL1; "MyText");
///
/// // Especifying saying that what is passed
/// kprint!(VGA; flush; "MyText");
///
/// // Using standard
/// kprint!("MyText");
/// ```
#[macro_export]
macro_rules! kprint {
    ($ctx:expr; $($arg:tt)*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            $ctx.lock().write_fmt(format_args!($($arg)*)).unwrap();
            // $ctx.lock().flush();
        });
    }};
    ($ctx:expr; flush; $($arg:tt)*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            $ctx.lock().write_fmt(format_args!($($arg)*)).unwrap();
            $ctx.lock().flush();
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
    ($fmt:expr) => ( $crate::kprintln!(SERIAL1; $fmt) );
    ($fmt:expr, $($arg:tt)*) => ( $crate::kprintln!(SERIAL1; $fmt, $($arg)*) );
}

/// Same as `kprint!`, but using `SERIAL1` as default output
#[macro_export]
macro_rules! s1print {
    ($($arg:tt)*) => ( $crate::kprint!(SERIAL1; $($arg)*) );
}

/// Printing macro that prints the same thing into `VGA` and `SERIAL1`
#[macro_export]
macro_rules! testprintln {
    ($fmt:expr) => {{
        $crate::s1println!($fmt);
        $crate::kprintln!($fmt);
    }};
    ($color:expr; $fmt:expr) => {{
        $crate::s1println!($fmt);
        $crate::vgacolor!($color);
        $crate::kprintln!($fmt);
        $crate::vgacolor!(Color::White);
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        $crate::s1println!($fmt, $($arg)*);
        $crate::kprintln!($fmt, $($arg)*);
    }};
    ($color:expr; $fmt:expr, $($arg:tt)*) => {{
        $crate::s1println!($fmt, $($arg)*);
        $crate::vgacolor!($color);
        $crate::kprintln!($fmt, $($arg)*);
        $crate::vgacolor!(Color::White);
    }};
}

/// Printing macro that prints the same thing into `VGA` and `SERIAL1`
#[macro_export]
macro_rules! testprint {
    ($fmt:expr) => {{
        $crate::s1print!($fmt);
        $crate::kprint!($fmt);
    }};
    ($color:expr; $fmt:expr) => {{
        $crate::s1print!($fmt);
        $crate::vgacolor!($color);
        $crate::kprint!($fmt);
        $crate::vgacolor!(Color::White);
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        $crate::s1print!($fmt, $($arg)*);
        $crate::kprint!($fmt, $($arg)*);
    }};
    ($color:expr; $fmt:expr, $($arg:tt)*) => {{
        $crate::s1print!($fmt, $($arg)*);
        $crate::vgacolor!($color);
        $crate::kprint!($fmt, $($arg)*);
        $crate::vgacolor!(Color::White);
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
