use super::ProcessorStatus;
use std::num::Wrapping;

pub struct Registers {
    pub counter: u16,
    pub stack: Wrapping<u8>,
    pub accumulator: Wrapping<u8>,
    pub x: Wrapping<u8>,
    pub y: Wrapping<u8>,
    pub status: ProcessorStatus,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            // see https://github.com/kpmiller/emulator101/blob/master/Chip8/chip8emu.c
            counter: 0x0200,
            stack: Wrapping(0xfa),
            accumulator: Wrapping(0),
            x: Wrapping(0),
            y: Wrapping(0),
            status: ProcessorStatus::new(),
        }
    }
}
