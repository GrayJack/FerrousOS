#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

use kernel::{
    self,
    uart::{PortAddress, SerialPort},
    vga::Vga,
    kprint,
    kprintln,
    init::gdt,
    init::idt
};


#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let slice = unsafe {
        core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
    };

    gdt::init().unwrap();
    idt::init().unwrap();

    let mut vga = Vga::new(slice);
    kprintln!(vga, "Hello Kernel World!!");

    let mut serial1 = SerialPort::new(PortAddress::COM1);
    serial1.init();
    kprintln!(vga, "Hello Serial: {:?}", serial1);

    loop {}
}
