use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable as Gdt, SegmentSelector},
        tss::TaskStateSegment as Tss,
    },
    VirtAddr,
};

use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// TSS is used on GDT, so makes sense putting it here instead of their own file.
lazy_static! {
    /// Default Task State Segment initialized.
    static ref TSS: Tss = {
        let mut tss = Tss::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr( unsafe { &STACK } );
            stack_start + STACK_SIZE
        };
        tss
    };
}

lazy_static! {
    /// Default Global Descriptor Table initialized.
    static ref GDT: (Gdt, Selectors) = {
        let mut gdt = Gdt::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (gdt, Selectors::new(code_selector, tss_selector))
    };
}

/// Loads the default Global Descriptor Table.
pub fn init() -> Result<(), &'static str> {
    use x86_64::instructions::{segmentation::set_cs, tables::load_tss};

    GDT.0.load();

    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }

    Ok(())
}

/// Holds the segments for kernel code and Task State Segment.
#[derive(Debug, Copy, Clone)]
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl Selectors {
    /// Create new instance of Selectors.
    pub fn new(code_selector: SegmentSelector, tss_selector: SegmentSelector) -> Selectors {
        Selectors {
            code_selector,
            tss_selector,
        }
    }
}
