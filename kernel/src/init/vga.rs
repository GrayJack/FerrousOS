use crate::vga::Vga;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// Default VGA output initialized
    pub static ref VGA: Mutex<Vga<&'static mut [u8]>> = {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };
        Mutex::new(Vga::new(slice))
    };
}
