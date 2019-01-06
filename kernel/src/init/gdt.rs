use x86_64::structures::gdt::{ 
    GlobalDescriptorTable,
    Descriptor
};

use lazy_static::lazy_static;

lazy_static! {
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();

        gdt.add_entry(Descriptor::kernel_code_segment());

        gdt
    };
}

pub fn init() -> Result<(), &'static str> {
    GDT.load();

    Ok(())
}
