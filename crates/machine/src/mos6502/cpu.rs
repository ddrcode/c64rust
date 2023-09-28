use super::{define_operations, OpsMap, Registers};

pub struct MOS6502 {
    pub registers: Registers,
    pub operations: OpsMap,
}

impl MOS6502 {
    pub fn new() -> Self {
        let mut operations = OpsMap::new();
        define_operations(&mut operations);

        MOS6502 {
            registers: Registers::new(),
            operations,
        }
    }
}
