use std::fmt;

// inspired by https://yizhang82.dev/nes-emu-cpu
// also see http://www.emulator101.com/6502-addressing-modes.html
#[derive(Debug, Copy, Clone)]
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
            Implicit => write!(f, "Implicit"),
            Accumulator => write!(f, "Accumulator"),
            Immediate => write!(f, "Immediate"),
            Relative => write!(f, "Relative"),
            ZeroPage => write!(f, "Zero Page"),
            ZeroPageX => write!(f, "Zero Page (X)"),
            ZeroPageY => write!(f, "Zero Page (Y)"),
            Absolute => write!(f, " Absolute"),
            AbsoluteX => write!(f, "Absolute X"),
            AbsoluteY => write!(f, "Absolute Y"),
            Indirect => write!(f, "Indirect"),
            IndirectX => write!(f, "Indirect X"),
            IndirectY => write!(f, "Indirect Y")
        }
    }
}
