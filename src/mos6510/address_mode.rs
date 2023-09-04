use std::fmt;

// inspired by https://yizhang82.dev/nes-emu-cpu
// also see http://www.emulator101.com/6502-addressing-modes.html
#[derive(Copy, Clone)]
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
        match self {
            AddressMode::Implicit => write!(f, "Implicit"),
            AddressMode::Accumulator => write!(f, "Accumulator"),
            AddressMode::Immediate => write!(f, "Immediate"),
            AddressMode::Relative => write!(f, "Relative"),
            AddressMode::ZeroPage => write!(f, "Zero Page"),
            AddressMode::ZeroPageX => write!(f, "Zero Page (X)"),
            AddressMode::ZeroPageY => write!(f, "Zero Page (Y)"),
            AddressMode::Absolute => write!(f, " Absolute"),
            AddressMode::AbsoluteX => write!(f, "Absolute X"),
            AddressMode::AbsoluteY => write!(f, "Absolute Y"),
            AddressMode::Indirect => write!(f, "Indirect"),
            AddressMode::IndirectX => write!(f, "Indirect X"),
            AddressMode::IndirectY => write!(f, "Indirect Y")
        }
    }
}
