#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

use kernel::{
    self,
    hlt_loop,
    uart::{PortAddress, SerialPort},
    kprintln,
    init::{
        gdt,
        idt,
        pic::PICS,
    }
};


#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init().unwrap();
    idt::init().unwrap();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    kprintln!("Hello Kernel World!!");

    // let mut serial1 = SerialPort::new(PortAddress::COM1);
    // serial1.init();
    // kprintln!("Hello Serial: {:?}", serial1);

    // invoke a breakpoint exception
    // x86_64::instructions::int3();

    // Trigger a page fault
    // Double fault exception
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // Trigger a stack overflow
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    //
    // stack_overflow();

    hlt_loop();
}
