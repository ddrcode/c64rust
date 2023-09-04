use super::{ Registers, define_operations, OpsMap, ProcessorStatus };

pub struct MOS6510 {
    pub registers: Registers,
    pub operations: OpsMap
}

impl MOS6510 {
    pub fn new() -> Self {
        let mut operations = OpsMap::new();
        define_operations(&mut operations);

        MOS6510 {
            registers: Registers::new(),
            operations: operations
        }
    }
}
