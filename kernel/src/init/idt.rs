use crate::{
    hlt_loop,
    vga::Color,
    init::{
        gdt,
        pic::{PICS, PIC_1_OFFSET},
        vga::VGA
    },
    kprintln,
    kprint,
};

use x86_64::{
    instructions::port::Port,
    structures::idt::{
        ExceptionStackFrame,
        InterruptDescriptorTable as Idt
    }
};

use lazy_static::lazy_static;

pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET; // 32

pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1; // 33

lazy_static! {
    /// Default Interrupt Descriptor Table initialized.
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // TODO Apparently, MINIX somehow deals with interrupts
        // dynamically. H O W?
        // Ideally, we should have a fn set_interrupt(n: u8, f: HandlerFunc)

        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);
        idt[usize::from(KEYBOARD_INTERRUPT_ID)].set_handler_fn(keyboard_interrupt_handler);

        // Needs unsafe for the set_stack_index method.
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

/// Loads the default Interrupt Descriptor Table.
pub fn init() -> Result<(), &'static str> {
    IDT.load();

    Ok(())
}

// Exception handler functions
// Idea behind it: Print the exeption and return to normal activity when possible.
// If happens to be not possible, print the exception and enter a infinite loop.

/// Divide by Zero exception handler
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    exception_info("DIVIDE BY ZERO", stack_frame);
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    exception_info("BREAKPOINT", stack_frame);
}

/// Double Fault exception handler
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    exception_info("DOUBLE FAULT", stack_frame);
    // Make sure we print only one in this case.
    // Since it is inrecoverable, it will keep getting the same error
    // and therefore, keep printing the same thing again and again and again...
    hlt_loop();
}

/// Time interrupt handler
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    kprint!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

/// Keyboard interrupt handler
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    let port = Port::new(0x60);
    let scancode: u8 = unsafe{ port.read() };
    kprint!("hello");
    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}

/// Helper function to the exception handler functions
fn exception_info(type_str: &str, stack_frame: &mut ExceptionStackFrame) {
    VGA.lock().set_foreground(Color::Red);
    kprintln!("EXCEPTION: {}\n{:#?}", type_str, stack_frame);
    VGA.lock().set_foreground(Color::Green);
}
