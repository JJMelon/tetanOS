#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
<<<<<<< HEAD
    println!("Hello World{}", "!");
    loop {}
=======
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
>>>>>>> main
}

#[panic_handler]
fn panic(_info: $PanicInfo) -> ! {
    loop {}
}

