extern "C" {
    readb() -> u8;
}

enum command {
    exit,
    none,
}

fn readline() -> &[u8; 80] {
    let mut byte: u8 = unsafe { readb() };
    let mut readbuf: [u8; 80] = [0; 80];
    let mut i: usize = 0;
    loop {
        match byte {
            0x0A => {
                break;
            },
            byte => {
                if (i < 80) {
                    readbuf[i] = byte;
                    i += 1;
                }
            }
        }
    }
    readbuf
}

fn tokenize(string: &[u8; 80]) -> command {
    let mut i: usize = 0;
    let mut buf: [u8; 80] = [0; 80];
    while (string[i] != 0x0) {
        buf[i] = string[i];
    }
    if (buf[..4] = [101, 120, 105, 116, 10]) {
        exit
    } else {
        none
    }
}
