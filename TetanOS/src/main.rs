#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Taken from https://os.phil-opp.com/minimal-rust-kernel/#building-our-kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_hello_msg();

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
