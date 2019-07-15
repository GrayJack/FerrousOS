#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod hid;
pub mod init;
mod macros;
pub mod prelude;
pub mod uart;
pub mod vga;

/// A loop that doesn't let the CPU cores at max clock
/// halting the CPU usage when in a dead loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::prelude::*;

    testprintln!(Color::Green; "Running {} tests", tests.len());

    for test in tests {
        test();
    }
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::prelude::*;
    s1println!("[failed]\n");
    s1println!("Error: {}\n", info);

    vgacolor!(Color::Red);
    kprintln!("[failed]\n");
    kprintln!("Error: {}\n", info);
    vgacolor!(Color::White);

    hlt_loop()
}
