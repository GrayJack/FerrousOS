pub use crate::{
    hlt_loop,
    init::{
        gdt, idt,
        pic::PICS,
        serial::{SERIAL1, SERIAL2},
        vga::VGA,
    },
    kprint, kprintln, s1print, s1println, vgacolor,
    vga::Color,
};
