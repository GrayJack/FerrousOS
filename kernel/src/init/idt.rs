use crate::{
    vga::Color,
    init::vga::VGA,
    kprintln,
};

use x86_64::structures::idt::{
    Entry,
    ExceptionStackFrame,
    HandlerFunc,
    InterruptDescriptorTable as Idt
};

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // TODO Apparently, MINIX somehow deals with interrupts
        // dynamically. H O W?
        // Ideally, we should have a fn set_interrupt(n: u8, f: HandlerFunc)

        let mut test_entry = Entry::<HandlerFunc>::missing();
        test_entry.set_handler_fn(divide_by_zero);

        idt.divide_by_zero = test_entry;

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn divide_by_zero(_: &mut ExceptionStackFrame) {
    panic!("Divide by zero");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    VGA.lock().set_foreground(Color::Red);
    kprintln!("CPU EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    VGA.lock().set_foreground(Color::Green);
}

pub fn init() -> Result<(), &'static str> {
    IDT.load();

    Ok(())
}
