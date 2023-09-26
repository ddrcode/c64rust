use super::ProcessorStatus;
use colored::*;
use std::fmt;
use std::num::Wrapping;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Registers {
    pub counter: u16,
    pub stack: Wrapping<u8>,
    pub accumulator: Wrapping<u8>,
    pub x: Wrapping<u8>,
    pub y: Wrapping<u8>,
    pub status: ProcessorStatus,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:${:04x}  {}:${:02x}  {}:${:02x}  {}:${:02x}  {}:${:02x}  {}:%{:08b}",
            "PC".bold(),
            self.counter,
            "A".bold(),
            self.accumulator,
            "X".bold(),
            self.x,
            "Y".bold(),
            self.y,
            "S".bold(),
            self.stack,
            "P".bold(),
            u8::from(&self.status)
        )
    }
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
