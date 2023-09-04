use crate::machine::{ RAM };
use crate::mos6510::{ MOS6510, Operation, Mnemonic, AddressMode, Operand, ProcessorStatus };

pub struct C64 {
    pub cpu: MOS6510,
    pub ram: RAM
}

impl C64 {
    pub fn new() -> Self {
        C64 {
            cpu: MOS6510::new(),
            ram: RAM::new(1<<16),
        }
    }

    // registry shortcuts
    fn A(&self) -> u8 { self.cpu.registers.accumulator } 
    fn X(&self) -> u8 { self.cpu.registers.x } 
    fn Y(&self) -> u8 { self.cpu.registers.y } 
    fn P(&self) -> ProcessorStatus { self.cpu.registers.status }
    fn PC(&self) -> u16 { self.cpu.registers.counter }
    fn SC(&self) -> u8 { self.cpu.registers.stack }

    // boot sequence, etc
    pub fn power_on(&mut self) {
        // see https://www.pagetable.com/c64ref/c64mem/
        self.ram.set_byte(0x0000, 0x2f); 
        self.ram.set_byte(0x0001, 0x37);
        self.ram.set_word(0x0003, 0xb1aa);
        self.ram.set_word(0x0005, 0xb391);
    }

    pub fn start(&mut self) {
        while self.next() {}
    }

    pub fn next(&mut self) -> bool {
        let op = self.decode_op(); 
        self.inc_counter();
        let operand = self.decode_operand(&op);
        (op.function)(&op, &operand, self);
        if let Mnemonic::BRK = op.mnemonic { false } else { true }
    }

    fn get_counter(&self) -> u16 {
        self.cpu.registers.counter
    }

    fn get_byte_for_counter(&self) -> u8 {
        let x = self.ram.get_byte(self.cpu.registers.counter);
        println!("get byte for counter ({}): {}", self.cpu.registers.counter,x);
        x
    }

    fn inc_counter(&mut self) {
        self.cpu.registers.counter += 1;
    } 

    fn decode_op(&self) -> Operation {
        let addr = self.get_counter();
        let opcode = self.get_byte_for_counter();
        match self.cpu.operations.get(&opcode) {
            Some(op) => *op,
            None => panic!("Opcode {:#04x} not found at address {:#06x}", opcode, addr)
        }
    }

    fn decode_operand(&mut self, op: &Operation) -> Operand {
        match op.address_mode {
            AddressMode::Implicit => Operand::None,
            _ => {
                let val = Operand::Byte(self.get_byte_for_counter());
                self.inc_counter();
                val
            }
        }
    }

    // see http://www.emulator101.com/6502-addressing-modes.html
    fn decode_address(&self, op: &Operation, operand: &Operand) -> Option<u16> {
        match op.address_mode {
            AddressMode::Absolute => operand.get_word(),
            AddressMode::AbsoluteX => Some(operand.get_word().unwrap() + self.X() as u16),
            AddressMode::AbsoluteY => Some(operand.get_word().unwrap() + self.Y() as u16),
            AddressMode::ZeroPage => Some(operand.get_byte().unwrap() as u16),
            AddressMode::ZeroPageX => Some((operand.get_byte().unwrap() + self.X()) as u16),
            AddressMode::Indirect => Some(self.ram.get_word(operand.get_word().unwrap())),
            _ => None
        }
    }

    pub fn load(&mut self, program: &[u8], addr: u16) {
        self.ram.write(addr, program);
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
