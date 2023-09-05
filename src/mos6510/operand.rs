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
