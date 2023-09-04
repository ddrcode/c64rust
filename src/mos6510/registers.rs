use super::{ ProcessorStatus };

pub struct Registers {
    pub counter: u16,
    pub stack: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub status: ProcessorStatus
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            // see https://github.com/kpmiller/emulator101/blob/master/Chip8/chip8emu.c
            counter: 0x0200,
            stack: 0xfa,
            accumulator: 0,
            x: 0,
            y: 0,
            status: ProcessorStatus::new()
        }
    }
}
