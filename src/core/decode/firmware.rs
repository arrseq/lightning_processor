// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS           | [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx xxxxxxxx]   | [x-------] [-x------] [--x-----] [---xxx--]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

pub struct Firmware {
    
}