#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use kernel::{
    vga::Vga,
    kprint,
    kprintln,
};

#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let slice = unsafe {
        core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
    };

    let mut vga = Vga::new(slice);

    kprintln!(vga, "Hello World");

    loop {}
}
