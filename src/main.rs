// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// disable name mangling bc otherwise the rust compiler
// will give this function a random name, and we need to
// tell the linker the kernel entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// called upon a kernel panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
