use core::{
    intrinsics,
    panic::PanicInfo,
};

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
