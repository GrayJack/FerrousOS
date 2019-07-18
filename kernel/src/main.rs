#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel::{self, mem::{self, *}, prelude::*};

use x86_64::{structures::paging::{mapper::MapperAllSizes, PageTable}, VirtAddr};
use bootloader::{BootInfo, entry_point};

#[cfg(not(test))]
pub mod panic;

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    gdt::init().unwrap();
    idt::init().unwrap();
    unsafe { PICS.lock().initialize() };
    // x86_64::instructions::interrupts::enable();

    kprintln!("Hello Kernel World!!");

    let mapper = unsafe { mem::init(boot_info.physical_memory_offset) };


    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        kprintln!("{:?} -> {:?}", virt, phys);
    }

    // let l4_table = unsafe { active_level4_table(boot_info.physical_memory_offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         kprintln!("L4 Entry {}: {:?}", i, entry);
    //
    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };
    //
    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 kprintln!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    #[cfg(test)]
    test_main();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

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

    // Page fault test
    // new
    // let ptr = 0x204370 as *mut u32;
    // read from a code page -> works
    // unsafe { let x = *ptr; }
    // write to a code page -> page fault
    // unsafe { *ptr = 42; }

    // Testing accessing page tables
    // let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;
    // let level_4_table = unsafe {&*level_4_table_ptr};
    // for i in 0..10 {
    //     kprintln!("Entry {}: {:?}", i, level_4_table[i]);
    // }

    hlt_loop();
}


pub fn test_runner(tests: &[&dyn Fn()]) {
    use kernel::prelude::*;

    testprintln!(Color::Green; "Running {} tests", tests.len());

    for test in tests {
        test();
    }
}
