use super::{AddressMode::*, Operand, OperationDef};
use std::fmt;

pub struct Operation {
    pub def: OperationDef,
    pub operand: Option<Operand>,
    pub address: Option<u16>,
}

impl Operation {
    pub fn new(def: OperationDef, operand: Option<Operand>, address: Option<u16>) -> Self {
        Operation {
            def: def,
            operand: operand,
            address: address,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = self.def.mnemonic.to_string();
        let o = if let Some(x) = &self.operand {
            x.to_string()
        } else {
            String::from("")
        };
        match &self.def.address_mode {
            Implicit => write!(f, "{}", m),
            Accumulator => write!(f, "{} A", m),
            Immediate => write!(f, "{} #${}", m, o),
            Relative => write!(f, "{} ${}", m, o),
            ZeroPage => write!(f, "{} ${}", m, o),
            ZeroPageX => write!(f, "{} ${}, X", m, o),
            ZeroPageY => write!(f, "{} ${}, Y", m, o),
            Absolute => write!(f, "{} ${}", m, o),
            AbsoluteX => write!(f, "{} ${}, X", m, o),
            AbsoluteY => write!(f, "{} ${}, Y", m, o),
            Indirect => write!(f, "{} (${})", m, o),
            IndirectX => write!(f, "{} (${}, X)", m, o),
            IndirectY => write!(f, "{}, (${}), Y", m, o),
        }
    }
}
