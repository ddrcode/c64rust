use std::num::Wrapping;
use crate::mos6510::{
    MOS6510, Operation, OperationDef, Mnemonic, AddressMode, Operand, ProcessorStatus
};
use super::{ Memory };

// see https://c64os.com/post/c64screencodes
const SCREEN_CODES: &str = "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
                            -·······························································\
                            @abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
                            -ABCDEFGHIJKLMNOPQRSTUVWXYZ·····································";
pub struct C64 {
    pub cpu: MOS6510,
    pub mem: Memory
}

pub trait RegSetter<T> {
    fn set_A(self, val: T);
    fn set_X(self, val: T);
    fn set_Y(self, val: T);
    fn set_SC(self, val: T);
}

impl RegSetter<u8> for &mut C64 {
    fn set_A(self, val: u8) { self.cpu.registers.accumulator = Wrapping(val); }
    fn set_X(self, val: u8) { self.cpu.registers.x = Wrapping(val); }
    fn set_Y(self, val: u8) { self.cpu.registers.y = Wrapping(val); }
    fn set_SC(self, val: u8) { self.cpu.registers.stack = Wrapping(val); }
}

impl RegSetter<Wrapping<u8>> for &mut C64 {
    fn set_A(self, val: Wrapping<u8>) { self.cpu.registers.accumulator = val; }
    fn set_X(self, val: Wrapping<u8>) { self.cpu.registers.x = val; }
    fn set_Y(self, val: Wrapping<u8>) { self.cpu.registers.y = val; }
    fn set_SC(self, val: Wrapping<u8>) { self.cpu.registers.stack = val; }
}

impl C64 {
    pub fn new() -> Self {
        C64 {
            cpu: MOS6510::new(),
            mem: Memory::new(None),
        }
    }

    // registry shortcuts
    pub fn A(&self) -> Wrapping<u8> { self.cpu.registers.accumulator }
    pub fn X(&self) -> Wrapping<u8> { self.cpu.registers.x }
    pub fn Y(&self) -> Wrapping<u8> { self.cpu.registers.y }
    pub fn A8(&self) -> u8 { self.cpu.registers.accumulator.0 }
    pub fn X8(&self) -> u8 { self.cpu.registers.x.0 }
    pub fn Y8(&self) -> u8 { self.cpu.registers.y.0 }
    pub fn A16(&self) -> u16 { self.cpu.registers.accumulator.0 as u16 }
    pub fn X16(&self) -> u16 { self.cpu.registers.x.0 as u16 }
    pub fn Y16(&self) -> u16 { self.cpu.registers.y.0 as u16 }
    pub fn P(&self) -> ProcessorStatus { self.cpu.registers.status }
    pub fn PC(&self) -> u16 { self.cpu.registers.counter }
    pub fn SC(&self) -> Wrapping<u8> { self.cpu.registers.stack }

    // boot sequence, etc
    pub fn power_on(&mut self) {
        // see https://www.pagetable.com/c64ref/c64mem/
        self.mem.set_byte(0x0000, 0x2f);
        self.mem.set_byte(0x0001, 0x37);
        self.mem.set_word(0x0003, 0xb1aa);
        self.mem.set_word(0x0005, 0xb391);

        // Location where BASIC program text is stored
        // https://www.pagetable.com/c64ref/c64mem/#002C
        self.mem.set_word(0x002b, 0x0801);

        // Highest address available to BASIC
        // see https://www.pagetable.com/c64ref/c64mem/#0037
        self.mem.set_word(0x0037, 0xa000);

        // graphics register
        // https://www.c64-wiki.com/wiki/53265
        self.mem.set_byte(0xd011, 0b00011000);

        // https://stackoverflow.com/questions/18811244/waiting-for-a-change-on-d012-c64-assembler
        // https://codebase64.org/doku.php?id=base:double_irq_explained
        self.mem.set_byte(0xd012, 0b11000001);
    }

    pub fn start(&mut self) {
        // while self.next() {}
        for i in 0..4000000 {
            if self.PC()==0xe5cf { break; }
            self.mem.set_byte(0xd012, (i%255)as u8);
            if !self.next() { break };
        }
    }

    pub fn next(&mut self) -> bool {
        let def = self.decode_op();
        let operand = self.decode_operand(&def);
        let address = if let Some(o)=&operand { self.decode_address(&def, &o) } else { None };
        let op = Operation::new(def, operand, address);
        self.print_op(&op);
        (def.function)(&op, self);
        Mnemonic::BRK != def.mnemonic
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
        let to_u16 = |a: u8, b: u8| -> (u16, u16) { (a as u16, b as u16) };
        match op.address_mode {
            AddressMode::Absolute => operand.get_word(),
            AddressMode::AbsoluteX => Some(operand.get_word().unwrap() + self.X16()),
            AddressMode::AbsoluteY => Some(operand.get_word().unwrap() + self.Y16()),
            AddressMode::ZeroPage => Some(operand.get_byte_as_u16().unwrap()),
            AddressMode::ZeroPageX => {
                let (o, x) = to_u16(operand.get_byte().unwrap(), self.X8());
                Some((o+x) & 0x00ff)
            },
            AddressMode::ZeroPageY => {
                let (o, y) = to_u16(operand.get_byte().unwrap(), self.Y8());
                Some((o+y) & 0x00ff)
            },
            AddressMode::Indirect => Some(self.mem.get_word(operand.get_word().unwrap())),
            AddressMode::IndirectX => {
                let (o, x) = to_u16(operand.get_byte().unwrap(), self.X8());
                let lo = self.mem.get_byte((o+x) & 0x00ff) as u16;
                let hi = u16::from(self.mem.get_byte((o+x+1) & 0x00ff)) << 8;
                Some(hi | lo)
            },
            AddressMode::IndirectY => {
                let (o, y) = to_u16(operand.get_byte().unwrap(), self.Y8());
                let lo = self.mem.get_byte(o) as u16;
                let hi = u16::from(self.mem.get_byte((o+1) & 0x00ff)) << 8;
                Some((hi | lo) + y)
            },
            AddressMode::Relative => {
                //  TODO verify that - o must be signed int (check notation)
                let (o, pc) = (operand.get_byte().unwrap() as i8, self.PC() as i64);
                Some(((pc + o as i64) & 0xffff) as u16)
            },
            _ => None
        }
    }

    pub fn push(&mut self, val: u8) {
        self.mem.set_byte(0x0100 | self.SC().0 as u16, val);
        self.cpu.registers.stack -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.cpu.registers.stack += 1;
        self.mem.get_byte(0x0100 | self.SC().0 as u16)
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.mem.write(addr, progmem);
    }

    pub fn run(&mut self, addr: u16) {
        self.cpu.registers.counter = addr;
        self.start();
    }

    pub fn print_screen(&self) {
        let chars: Vec<char> = SCREEN_CODES.chars().collect();
        let mut n = 0;
        println!();
        for i in 0x0400..0x07e8 {
            let sc = self.mem.get_byte(i) as usize;
            print!("{}", chars[sc]);
            n += 1;
            if n % 40 == 0 { println!() };
        }
        println!();
    }

    // utility functions

    /// Returns current stack memory address
    pub fn stack_addr(&self) -> u16 {
        0x0100 | self.SC().0 as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asm_test(prog: &[u8], res: u8, flags: u8) {
        let mut c64 = C64::new();
        c64.cpu.registers.counter = 0x0300;
        c64.load(prog, 0x0300);
        c64.next();
        c64.next();
        assert_eq!(res, c64.A8());
        assert_eq!(flags, u8::from(&c64.P())); // NV1BDIZC
    }

    #[test]
    fn test_adc() {
        asm_test(&[0xa9, 0x50, 0x69, 0x10], 0x60, 0b00100000);
        asm_test(&[0xa9, 0x50, 0x69, 0x50], 0xa0, 0b11100000);
        asm_test(&[0xa9, 0x50, 0x69, 0x90], 0xe0, 0b10100000);
        asm_test(&[0xa9, 0x50, 0x69, 0xd0], 0x20, 0b00100001);
        asm_test(&[0xa9, 0xd0, 0x69, 0x10], 0xe0, 0b10100000);
        asm_test(&[0xa9, 0xd0, 0x69, 0x50], 0x20, 0b00100001);
        asm_test(&[0xa9, 0xd0, 0x69, 0x90], 0x60, 0b01100001);
        asm_test(&[0xa9, 0xd0, 0x69, 0xd0], 0xa0, 0b10100001);
    }

    #[test]
    fn test_sbc() {
        asm_test(&[0xa9, 0xf0, 0xe9, 0x07], 0xf0-0x07, 0b10100001);
        // scenarios from: https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        asm_test(&[0xa9, 0x50, 0xe9, 0xf0], 0x60, 0b00100000);
        asm_test(&[0xa9, 0x50, 0xe9, 0xb0], 0xa0, 0b11100000);
        asm_test(&[0xa9, 0x50, 0xe9, 0x70], 0xe0, 0b10100000);
        asm_test(&[0xa9, 0x50, 0xe9, 0x30], 0x20, 0b00100001);
        asm_test(&[0xa9, 0xd0, 0xe9, 0xf0], 0xe0, 0b10100000);
        asm_test(&[0xa9, 0xd0, 0xe9, 0xb0], 0x20, 0b00100001);
        asm_test(&[0xa9, 0xd0, 0xe9, 0x70], 0x60, 0b01100001);
        asm_test(&[0xa9, 0xd0, 0xe9, 0x30], 0xa0, 0b10100001);
    }
}

