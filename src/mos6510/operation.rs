use super::{ OperationDef, Operand };

pub struct Operation {
    pub def: OperationDef,
    pub address: Option<u16>,
    pub operand: Option<Operand>
}

impl Operation {
    pub fn new(def: OperationDef) -> Self {
        Operation {
            def: def,
            address: None,
            operand: None
        }
    }
}
