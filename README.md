# TetanOS
This is the repo for TetanOS, a group project building an simple OS using as much Rust as possible for the CMU Course 98-008: Shilling The Rust Programming Language.

## Bootloader:

Requires two stages: 
- Tiny Stage0 just Loads larger Stage1 
- Stage 1 enters protected mode, prepares system for kernel, then loads the kernel.

## Kernel:

## References:
- A simple operating system written in rust https://os.phil-opp.com/
- Advanced MSDOS Programming 2nd Edition
- Course notes from CMU Course 15-410: Operating System Design and Implementation https://www.cs.cmu.edu/~410-s07/p4/p4-boot.pdf
-(Mostly) Rust Bootloader from phil-op https://github.com/rust-osdev/bootloader/tree/rewrite - FOR CHERRY ON TOP FULL RUST
- Much simpler i386 bootloader https://github.com/chuang76/x86-bootloader

### Filesystem
- FAT file system https://www.sobyte.net/post/2022-01/rust-fat32/
