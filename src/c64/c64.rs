use crate::mos6510::{
    MOS6510, Operation, OperationDef, Mnemonic, AddressMode, Operand, ProcessorStatus
};
use super::{ Memory };

pub struct C64 {
    pub cpu: MOS6510,
    pub mem: Memory
}

impl C64 {
    pub fn new() -> Self {
        C64 {
            cpu: MOS6510::new(),
            mem: Memory::new(None),
        }
    }

    // registry shortcuts
    pub fn A(&self) -> u8 { self.cpu.registers.accumulator }
    pub fn X(&self) -> u8 { self.cpu.registers.x }
    pub fn Y(&self) -> u8 { self.cpu.registers.y }
    pub fn P(&self) -> ProcessorStatus { self.cpu.registers.status }
    pub fn PC(&self) -> u16 { self.cpu.registers.counter }
    pub fn SC(&self) -> u8 { self.cpu.registers.stack }

    // boot sequence, etc
    pub fn power_on(&mut self) {
        // see https://www.pagetable.com/c64ref/c64mem/
        self.mem.set_byte(0x0000, 0x2f);
        self.mem.set_byte(0x0001, 0x37);
        self.mem.set_word(0x0003, 0xb1aa);
        self.mem.set_word(0x0005, 0xb391);
    }

    pub fn start(&mut self) {
        while self.next() {}
    }

    pub fn next(&mut self) -> bool {
        let def = self.decode_op();
        let operand = self.decode_operand(&def);
        let address = if let Some(o)=&operand { self.decode_address(&def, &o) } else { None };
        let op = Operation::new(def, operand, address);
        self.print_op(&op);
        (def.function)(&op, self);
        if Mnemonic::BRK == def.mnemonic { false } else { true }
    }

    fn print_op(&self, op: &Operation) {
        let addr = self.PC() - op.def.len() as u16;
        let val = match op.def.len() {
            2 => format!("{:02x}   ", self.mem.get_byte(addr+1)),
            3 => format!("{:02x} {:02x}", self.mem.get_byte(addr+1), self.mem.get_byte(addr+2)),
            _ => String::from("     ")
        };
        println!("{:04x}: {:02x} {} | {}", addr, op.def.opcode, val, op);
    }

    fn get_byte_and_inc_pc(&mut self) -> u8 {
        let val = self.mem.get_byte(self.PC());
        self.inc_counter();
        val
    }

    fn get_word_and_inc_pc(&mut self) -> u16 {
        let val = self.mem.get_word(self.PC());
        self.inc_counter();
        self.inc_counter();
        val
    }

    fn inc_counter(&mut self) {
        self.cpu.registers.counter += 1;
    }

    fn decode_op(&mut self) -> OperationDef {
        let opcode = self.get_byte_and_inc_pc();
        match self.cpu.operations.get(&opcode) {
            Some(op) => *op,
            None => panic!("Opcode {:#04x} not found at address {:#06x}", opcode, self.PC()-1)
        }
    }

    fn decode_operand(&mut self, op: &OperationDef) -> Option<Operand> {
        match op.operand_len() {
            0 => None,
            1 => Some(Operand::Byte(self.get_byte_and_inc_pc())),
            2 => Some(Operand::Word(self.get_word_and_inc_pc())),
            _ => panic!("Invalid operand length")
        }
    }

    // see http://www.emulator101.com/6502-addressing-modes.html
    fn decode_address(&self, op: &OperationDef, operand: &Operand) -> Option<u16> {
        match op.address_mode {
            AddressMode::Absolute => operand.get_word(),
            AddressMode::AbsoluteX => Some(operand.get_word().unwrap() + self.X() as u16),
            AddressMode::AbsoluteY => Some(operand.get_word().unwrap() + self.Y() as u16),
            AddressMode::ZeroPage => Some(operand.get_byte().unwrap() as u16),
            AddressMode::ZeroPageX => Some((operand.get_byte().unwrap() + self.X()) as u16),
            AddressMode::ZeroPageY => Some((operand.get_byte().unwrap() + self.Y()) as u16),
            AddressMode::Indirect => Some(self.mem.get_word(operand.get_word().unwrap())),
            AddressMode::IndirectX => panic!("Not implemented"),
            AddressMode::IndirectY => panic!("Not implemented"),
            AddressMode::Relative => Some(self.PC() + operand.get_byte().unwrap() as u16),
            _ => None
        }
    }

    pub fn push(&mut self, val: u8) {
        self.mem.set_byte(0x100+self.SC() as u16, val);
        self.cpu.registers.stack -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.cpu.registers.stack += 1;
        self.mem.get_byte(0x100+self.SC() as u16)
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.mem.write(addr, progmem);
    }

    pub fn run(&mut self, addr: u16) {
        self.cpu.registers.counter = addr;
        self.start();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut c64 = C64::new();
        c64.load(&[0x69, 0x05, 0x69, 0x07], 0x0100);
        c64.next();
        assert_eq!(0x0100, c64.PC());
    }
}
