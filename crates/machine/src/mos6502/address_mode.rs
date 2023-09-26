use std::fmt;
use serde_derive::Serialize;

// inspired by https://yizhang82.dev/nes-emu-cpu
// also see http://www.emulator101.com/6502-addressing-modes.html
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
