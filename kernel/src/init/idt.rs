use crate::{
    vga::Color,
    init::vga::VGA,
    kprintln,
};

use x86_64::structures::idt::{
    ExceptionStackFrame,
    InterruptDescriptorTable as Idt
};

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // TODO Apparently, MINIX somehow deals with interrupts
        // dynamically. H O W?
        // Ideally, we should have a fn set_interrupt(n: u8, f: HandlerFunc)

        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init() -> Result<(), &'static str> {
    IDT.load();

    Ok(())
}

// Exception handler functions
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    exception_info("DIVIDE BY ZERO", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    exception_info("BREAKPOINT", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    exception_info("DOUBLE FAULT", stack_frame);
    // Make sure we print only one in this case.
    // Since it is inrecoverable, it will keep getting the same error
    // and therefore, keep printing the same thing again and again and again...
    loop {}
}

fn exception_info(type_str: &str, stack_frame: &mut ExceptionStackFrame) {
    VGA.lock().set_foreground(Color::Red);
    kprintln!("EXCEPTION: {}\n{:#?}", type_str, stack_frame);
    VGA.lock().set_foreground(Color::Green);
}
