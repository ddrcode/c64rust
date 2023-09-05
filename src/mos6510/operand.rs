use std::fmt;
pub enum Operand {
    Byte(u8),
    Word(u16)
}

impl Operand {
    pub fn get_byte(&self) -> Option<u8> {
        match self {
            Operand::Byte(val) => Some(*val),
            _ => None
        }
    }

    pub fn get_word(&self) -> Option<u16> {
        match self {
            Operand::Word(val) => Some(*val),
            _ => None
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Byte(x) => write!(f, "{:02x}", x),
            Operand::Word(x) => write!(f, "{:04x}", x)
        }
    }
}

