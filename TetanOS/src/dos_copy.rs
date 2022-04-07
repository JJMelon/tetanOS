/*
 * This file contians functions and structs that are basically translated
 * from the STDDOS.ASM and MSDOS.ASM v1.25 file source code. The style for
 * this will be awful as my competency in Rust is garbage lol
 *
 * Also note: My comments will start with ////
 */

/* Start of STDDOS.ASM */

// Use the switches below to produce the standard Microsoft version of the
// IBM version of the operating system

const MSVER = true;
const IBM = true; //// in original code this is set to false

// Set this switch to cause DOS to move itself to the end of memory

const HIGHMEM = false;

// Turn on switch below to allow testing disk code with DEBUG. It sets
// up a different stack for disk I/O (functions > 11) than that used for
// character I/O which effectively makes the DOS re-entrant.

const DSKTEST = false;

/* End of STDDOS.ASM */

/* Start of MSDOS.ASM */

// These things here are basically C preprocessor directves and will be
// modified later to have the same effect

if (IBM) {
   let escch = 0x0;
   let cancel = 0x1B;   // Cancel with escape
   let toglins = true;  // One key toggles insert mode
   let togprln = true;  // One key toggles printer echo
   let numdev = 0x6;    // Include "COM1" as I/O device name
   let zeroext = true;
} else {
   let escch = 0x1B;
   let cancel = 0x18;   // Cancel with escape
   let toglins = false; // One key toggles insert mode
   let togprln = false; // One key toggles printer echo
   let numdev = 0x5;    // Include "COM1" as I/O device name
   let zeroext = false;
}

//// I have no idea what these do lol
const maxcall = 36;
const maxcom = 46;
const intbase = 0x80;
const inttab = 0x20;
const entrypointseg = 0xC;
const entrypoint = intbase + 0x40;
const contc = inttab + 3;
const exit = intbase + 8;
const longjmp = 0xEA;
const longcall = 0x9A;
const maxdif = 0xFFF;
const saveexit = 10;

// Field definition for FCBs

//// FCB stands for File Control Block, which is a file system structure
//// in which the state of an open file is maintained

struct fcblock {
    //// This is initially unnamed
    drive_code_name: &[u8, 12], // Drive code and name
    extent: u16,
    recsiz: u16,    // Sice of record (usersettable)
    filsiz: u16,    // Size of file in bytes
    drvbp:  u16,    // BP for SEARCH FIRST and SEARCH NEXT
    fdate:  u16,    // Date of last writing
    ftime:  u16,    // Time of last writing
    devid:  u16,    // Device ID number bits 0-5
                    // bit 7=0 for file, bit 7=1 for I/O device
                    // If file, bit 6=0 if dirty
                    // If I/O device, bit 6=0 if EOF (input)
    firclus: u16,   // First cluster of file
    lstclus: u16,   // Last cluster accessed
    clustpos: u16,  // Position of last cluster accessed
    _: u8,  // Forced NR offset 32
    nr: u8, // Next record
    rr: &[u8, 3],   // Random record
    fildirent: filsiz,  // Used only by SEARCH FIRST AND SEARCH NEXT
}

/* End of MSDOS.ASM */
