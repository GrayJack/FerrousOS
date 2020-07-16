use crate::{init::pic::PIC_1_OFFSET, prelude::*};

use x86_64::{
    instructions::port::Port,
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable as Idt, InterruptStackFrame, PageFaultErrorCode},
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

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(dev_not_available_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(seg_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_handler);
        idt.general_protection_fault.set_handler_fn(protection_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        // idt.double_fault.set_handler_fn(double_fault_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        idt.simd_floating_point.set_handler_fn(simd_float_handler);
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
extern "x86-interrupt" fn divide_error_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("DIVIDE BY ZERO", stack_frame);
}

/// Non Maskable Interrupt exception handler
extern "x86-interrupt" fn non_maskable_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("NON MASKABLE INTERRUPT", stack_frame);
}

/// Debug exception handler
extern "x86-interrupt" fn debug_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("DEBUG", stack_frame);
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("BREAKPOINT", stack_frame);
}

/// Overflow exception handler
extern "x86-interrupt" fn overflow_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("OVERFLOW", stack_frame);
}

/// Bound Range Exceeded exception handler
extern "x86-interrupt" fn bound_range_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("BOUND RANGE EXCEEDED", stack_frame);
}

/// Invalid Opcode exception handler
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("INVALID OPTICODE", stack_frame);
}

/// Device Not Available exception handler
extern "x86-interrupt" fn dev_not_available_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("DEVICE NOT AVAILABLE", stack_frame);
}

/// Device Not Available exception handler
extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("X87 FLOATING POINT", stack_frame);
}

/// Machine Check exception handler
extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut InterruptStackFrame) -> ! {
    exception_info("X87 MACHINE CHECK", stack_frame);
    // Make sure we print only one in this case.
    // Since it is inrecoverable, it will keep getting the same error
    // and therefore, keep printing the same thing again and again and again...
    hlt_loop();
}

/// SIMD Floating Point exception handler
extern "x86-interrupt" fn simd_float_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("SIMD FLOATING POINT", stack_frame);
}

/// Virtualization exception handler
extern "x86-interrupt" fn virtualization_handler(stack_frame: &mut InterruptStackFrame) {
    exception_info("VIRTUALIZATION", stack_frame);
}

/// Double Fault exception handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: DOUBLE FAULT");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);
    vgacolor!(Color::White);
    // Make sure we print only one in this case.
    // Since it is inrecoverable, it will keep getting the same error
    // and therefore, keep printing the same thing again and again and again...
    hlt_loop();
}

/// Invalid TSS exception handler
extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: INVALID TSS");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Segment Not Present exception handler
extern "x86-interrupt" fn seg_not_present_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: SEGMENT NOT PRESENT");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Stack Segment Fault exception handler
extern "x86-interrupt" fn stack_segment_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: STACK SEGMENT FAULT");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// General Protection Fault exception handler
extern "x86-interrupt" fn protection_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: GENERAL PROTECTION FAULT");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Alignment Check exception handler
extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: ALIGNMENT CHECK");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Security Exception exception handler
extern "x86-interrupt" fn security_exception_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: ALIGNMENT CHECK");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Page fault handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: PAGE FAULT");
    kprintln!("Error code: {:?}", error_code);
    kprintln!("Accessed Address: {:?}", Cr2::read());
    kprintln!("{:#?}", stack_frame);

    vgacolor!(Color::White);
    hlt_loop();
}

/// Time interrupt handler
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    // kprint!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

/// Keyboard interrupt handler
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                layouts::Us104Key,
                ScancodeSet1,
                HandleControl::MapLettersToUnicode
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => kprint!("{}", character),
                DecodedKey::RawKey(key) => kprint!("{:?}", key),
            }
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}

/// Helper function to the exception handler functions
fn exception_info(type_str: &str, stack_frame: &mut InterruptStackFrame) {
    vgacolor!(Color::Red);
    kprintln!("EXCEPTION: {}\n{:#?}", type_str, stack_frame);
    vgacolor!(Color::Green);
}
