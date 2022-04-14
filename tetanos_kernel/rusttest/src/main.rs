#![no_std]
#![no_main]

use core::panic::PanicInfo;

// #[no_mangle]
// pub extern "C" fn _start() -> () {
//   ();
// }


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}