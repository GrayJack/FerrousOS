use x86_64::structures::idt::{
    InterruptDescriptorTable,
    Entry,
    HandlerFunc,
    ExceptionStackFrame
};

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // TODO Apparently, MINIX somehow deals with interrupts
        // dynamically. H O W?
        // Ideally, we should have a fn set_interrupt(n: u8, f: HandlerFunc)
        
        let mut test_entry = Entry::<HandlerFunc>::missing();
        test_entry.set_handler_fn(divide_by_zero);

        idt.divide_by_zero = test_entry;

        idt
    };
}

extern "x86-interrupt" fn divide_by_zero(_: &mut ExceptionStackFrame) {
    panic!("Divide by zero");
}

pub fn init() -> Result<(), &'static str> {
    IDT.load();

    Ok(())
}
