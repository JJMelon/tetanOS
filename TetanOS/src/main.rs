#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        print!("> ");

        let mut input = readline();
        let command = tokenize(input);
            match command {
                exit => {
                    println!("Goodbye");
                },
                none => {
                    println!("not implemented");
                },
            }
        }
    }
}

#[panic_handler]
fn panic(_info: $PanicInfo) -> ! {
    loop {}
}

