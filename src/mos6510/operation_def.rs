use super::{AddressMode, AddressMode::*, Mnemonic};

#[derive(Clone)]
pub struct OperationDef {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub cycles: u8,
    pub page_boundary_cycle: bool,
    pub address_mode: AddressMode,
    pub fn_name: String, // pub function: OpFn,
}

impl OperationDef {
    pub fn len(&self) -> u8 {
        match self.address_mode {
            Implicit | Accumulator => 1,
            Immediate | Relative | ZeroPage | ZeroPageX | ZeroPageY | IndirectX | IndirectY => 2,
            Absolute | AbsoluteX | AbsoluteY | Indirect => 3,
        }
    }

    pub fn operand_len(&self) -> u8 {
        self.len() - 1
    }
}
