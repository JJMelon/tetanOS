#![no_std]
#![no_main]

// create my own print in assembly to take advantage of BIOS interrupts

extern "C" {
    fn read_b() -> u8;
}

pub extern "C" fn read_nb() -> &str {
    let mut ascii_char = read_b();
}
