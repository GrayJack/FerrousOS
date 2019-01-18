use core::{
    intrinsics,
    panic::PanicInfo,
};

use kernel::kprintln;

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{:?}", info);
    unsafe { intrinsics::abort() }
}
