#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {
        unsafe {
            asm!(
                "mov ah, 0x00",
                "2:",
                "mov ah,0x00",
                "int 0x16",
                "mov ah,0x0e",
                "int 0x10",
                "jmp 2b",
            );
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

