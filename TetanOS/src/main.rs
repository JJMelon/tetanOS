#![no_std]
#![no_main]

static HELLO: &[u8] = b"Hello World!";

// Taken from https://os.phil-opp.com/minimal-rust-kernel/#building-our-kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let color = 0xb; // Cyan

    // Copy HELLO string into VGA buffer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color;
        }
    }

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
