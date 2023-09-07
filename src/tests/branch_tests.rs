use super::asm_test;
use crate::c64::*;

#[test]
fn test_bpl() {
    //                                                NV1BDIZC
    asm_test(
        &[0xa9, 0x00, 0x10, 0x01, 0x00, 0xa9, 0xff],
        0xff,
        0b10110000,
    );
    asm_test(
        &[
            0xa9, 0x00, // LDA #$00
            0x10, 0x04, // BPL +4
            0x00, // BRK
            0xa9, 0x10, // LDA #$10
            0x00, // BRK
            0x10, 0xfb, // BPL -3
            0xa9, 0xff, // LDA #$ff
        ],
        0x10,
        0b00110000,
    );
}
