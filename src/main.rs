// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// disable name mangling bc otherwise the rust compiler
// will give this function a random name, and we need to
// tell the linker the kernel entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // cast to a raw ptr
    let vga_buffer = 0xb8000 as *mut u8;
    
    // itr over chars, w/ enumeration
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // write char & cyan color to vga buffer address
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// called upon a kernel panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
