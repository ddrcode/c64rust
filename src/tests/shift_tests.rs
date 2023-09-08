use super::asm_test;
use crate::c64::*;

#[test]
fn test_asl() {
    //                                    NV1BDIZC
    asm_test(&[0xa9, 0x03, 0x0a], 0x06, 0b00110000);
    asm_test(&[0xa9, 0xff, 0x0a], 0xfe, 0b10110001);
    asm_test(&[0xa9, 0x00, 0x0a], 0x00, 0b00110010);
}

#[test]
fn test_lsr() {
    //                                    NV1BDIZC
    asm_test(&[0xa9, 0x03, 0x4a], 0x01, 0b00110001);
    asm_test(&[0xa9, 0xff, 0x4a], 0x7f, 0b00110001);
    asm_test(&[0xa9, 0x00, 0x4a], 0x00, 0b00110010);
}

#[test]
fn test_rol() {
    //                                    NV1BDIZC
    asm_test(&[0xa9, 0x03, 0x2a], 0x06, 0b00110000);
    asm_test(&[0xa9, 0xff, 0x2a], 0xfe, 0b10110001);
    asm_test(&[0xa9, 0x00, 0x2a], 0x00, 0b00110010);

    // with carry
    asm_test(&[0x38, 0xa9, 0x00, 0x2a], 1, 0b00110000);
}

#[test]
fn test_ror() {
    //                                    NV1BDIZC
    asm_test(&[0xa9, 0x03, 0x6a], 0x01, 0b00110001);
    asm_test(&[0xa9, 0xff, 0x6a], 0x7f, 0b00110001);
    asm_test(&[0xa9, 0x00, 0x6a], 0x00, 0b00110010);

    // with carry
    asm_test(&[0x38, 0xa9, 0x00, 0x6a], 0x80, 0b10110000);
}
