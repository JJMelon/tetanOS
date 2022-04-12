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
    //// This array is initially unnamed
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
    _: u8,          // Forced NR offset 32
    nr: u8,         // Next record
    rr: &[u8, 3],       // Random record
    fildirent: filsiz,  // Used only by SEARCH FIRST AND SEARCH NEXT
}

// Description of 32-byte directory entry (same as returned by SEARCH FIRST
// and SEARCH NEXT, functions 17 and 18).
//
// Location      bytes   Description
//
//    0          11      File name and extension ( 0E5H if empty)
//   11           1      Attributes. Bits 1 or 2 make file hidden
//   12          10      Zero field (for expansion)
//   22           2      Time. Bits 0-4=seconds/2, bits 5-10=minute,
//                       11-15=hour
//   24           2      Date. Bits 0-4=day, bits 5-8=month,
//                       bits 9-15=year-1980
//   26           2      First allocation unit ( < 4080 )
//   28           4      File size, in bytes (LSB first, 30 bits max.)
//
// The File Allocation Table uses a 12-bit entry for each allocation unit on
// the disk. These entries are packed, two for every three bytes. The
// contents of entry number N is found by 1) multiplying N by 1.5; 2)
// adding the result to the base address of the Allocation Table; 3)
// fetching the 16-bit word at this address; 4) If N was odd (so that N*1.
// was not an integer), shift the word right four bits; 5) mask to 12 bits
// (AND with 0FFF hex). Entry number zero is used as an end-of-file trap in
// the OS and as a flag for directory entry size (if SMALLDIR selected).
// Entry 1 is reserved for future use. The first available allocation unit
// is assigned entry number two, and even though it is the first, is called
// cluster 2. Entries greater than 0FF8H are end of file marks; entries of
// zero are unallocated. Otherwise, the contents of a FAT entry is the
// number of the next cluster in the file.

struct dpblock {
    devnum: u8,     // I/O driver number
    drvnum: u8,     // Physical Unite number
    secsiz: u16,    // Size of physical sector in bytes
    clusmsk: u8,    // Sectors/cluster - 1
    clusshift: u8,  // Log2 of sectors/cluster
    firfat: u16,    // Starting record of FATs
    fatcnt: u8,     // Number of FATs for this drive
    maxent: u16,    // Number of directory entries
    firrec: u16,    // First sector of first cluster
    maxclus: u16,   // Number of clusters on drive + 1
    fatsiz: u8,     // Number of records occupied by FAT
    firdir: u16,    // Starting record of directory
    fat: u16,       // Pointer to start of FAT
}

const dpbsiz = 20; // Size of the structure in bytes
static dirsec = firrec; // Number of dir. sectors (init temporary)
static dkssiz = maxclus; // Size of disk (temp used during init only)

// The following are all of the segments used
// They are declared in the order that they should be placed in the
// executable

//// I have no idea how these work in an assembler, so I'm just going to
//// comment them out. This can probably be ignored until we get this file
//// to become assembly, so they'll still remained commented out

//// CODE   SEGMENT
//// CODE   ENDS
////
//// CONSTANTS  SEGMENT BYTE
//// CONSTANTS  ENDS
////
//// DATA   SEGMENT WORD
//// DATA   ENDS
////
//// DOSGROUP   GROUP   CODE,CONSTANTS,DATA
////
//// SEGBIOS SEGMENT
//// SEGBIOS END

// BIOS entry point definitions
if (IBM) {
    BIOSSEG = 0x60;
} else {
    BIOSSEG = 0x40;
}

//// Now for some more assembler directives
//// For the BIOS segment

/*
 SEGBIOS         SEGMENT AT BIOSSEG
                ORG     0
                DB      3 DUP (?)       ;Reserve room for jump to init code
BIOSSTAT        DB      3 DUP (?)       ;Console input status check
BIOSIN          DB      3 DUP (?)       ;Get console character
BIOSOUT         DB      3 DUP (?)       ;Output console character
BIOSPRINT       DB      3 DUP (?)       ;Output to printer
BIOSAUXIN       DB      3 DUP (?)       ;Get byte from auxilliary
BIOSAUXOUT      DB      3 DUP (?)       ;Output byte to auxilliary
BIOSREAD        DB      3 DUP (?)       ;Disk read
BIOSWRITE       DB      3 DUP (?)       ;Disk write
BIOSDSKCHG      DB      3 DUP (?)       ;Dsik-change status
BIOSSETDATE     DB      3 DUP (?)       ;Set date
BIOSSETTIME     DB      3 DUP (?)       ;Set time
BIOSGETTIME     DB      3 DUP (?)       ;Get time and date
BIOSFLUSH       DB      3 DUP (?)       ;Clear console input buffer
BIOSMAPDEV      DB      3 DUP (?)       ;Dynamic disk table mapper

SEGBIOS ENDS
*/
// Location of user registers relative user stack pointers
/*
STKPTRS STRUC
AXSAVE  DW      ?
BXSAVE  DW      ?
CXSAVE  DW      ?
DXSAVE  DW      ?
SISAVE  DW      ?
DISAVE  DW      ?
BPSAVE  DW      ?
DSSAVE  DW      ?
ESSAVE  DW      ?
IPSAVE  DW      ?
CSSAVE  DW      ?
FSAVE   DW      ?
STKPTRS ENDS

//// Do I even need the above struct? I guess I'll find out
*/

// Start of code

//// CODE   SEGMET
//// ASSUME CS:DOSGROUP,DS:DOSGROUP,ES:DOSGROUP,SS:DOSGROUP

//// ORG    0
//// CODSTRT EQU    $
//// JMP DOSINIT

//// Am just gonna copy paste some more weird assembler code until I get to
//// something that I can implement as a function in Rust
/*
ESCCHAR DB      ESCCH   ;Lead-in character for escape sequences
ESCTAB:
        IF      NOT IBM
        DB      "S"     ;Copy one char
        DB      "V"     ;Skip one char
        DB      "T"     ;Copy to char
        DB      "W"     ;Skip to char
        DB      "U"     ;Copy line
        DB      "E"     ;Kill line (no change in template)
        DB      "J"     ;Reedit line (new template)
        DB      "D"     ;Backspace
        DB      "P"     ;Enter insert mode
        DB      "Q"     ;Exit insert mode
        DB      "R"     ;Escape character
        DB      "R"     ;End of table
        ENDIF
        IF      IBM
        DB      64      ;Crtl-Z - F6
        DB      77      ;Copy one char - -->
        DB      59      ;Copy one char - F1
        DB      83      ;Skip one char - DEL
        DB      60      ;Copy to char - F2
        DB      62      ;Skip to char - F4
        DB      61      ;Copy line - F3
        DB      61      ;Kill line (no change to template ) - Not used
        DB      63      ;Reedit line (new template) - F5
        DB      75      ;Backspace - <--
        DB      82      ;Enter insert mode - INS (toggle)
        DB      65      ;Escape character - F7
        DB      65      ;End of table
        ENDIF

ESCTABLEN EQU   $-ESCTAB
        IF      NOT IBM
HEADER  DB      13,10,"MS-DOS version 1.25"
        IF      HIGHMEM
        DB      "H"
        ENDIF
        IF      DSKTEST
        DB      "D"
        ENDIF

        DB      13,10
        DB      "Copyright 1981,82 Microsoft, Inc.",13,10,"$"
        ENDIF
*/

//// Finally, time to shine

fn quit() {
    unsafe {
        asm!("mov ah, 0",);
    };
    savregs()
}

fn command() -> fn { // Interrupt call entry point
    if ((ah as u8) <= maxcom) {
        savregs()
    } else {
        badcall()
    }
}

fn badcall() -> () {
    unsafe {
        asm!("cmp ah, {maxcom}",);
    };
}

fn entry() ->


/* End of MSDOS.ASM */
