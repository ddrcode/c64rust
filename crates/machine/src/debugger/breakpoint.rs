use serde_derive::Deserialize;

use crate::machine::{Addr, Machine};
use crate::mos6502::{Mnemonic, Operation};

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum Breakpoint {
    Address(Addr),
    Interrupt,
    Instruction(Mnemonic),
    BRK,
    Opcode(u8),
    Byte((Addr, u8)),
}

impl Breakpoint {
    pub fn applies<M: Machine>(&self, op: &Operation, machine: &M) -> bool {
        match *self {
            Self::Address(a) => a == machine.PC(),
            Self::Interrupt => panic!("Interrupt breakpoint not implemented!"),
            Self::Instruction(m) => op.def.mnemonic == m,
            Self::BRK => op.def.mnemonic == Mnemonic::BRK,
            Self::Opcode(o) => op.def.opcode == o,
            Self::Byte((addr, val)) => {
                addr == machine.PC() && val == machine.get_byte(machine.PC())
            }
        }
    }
}
