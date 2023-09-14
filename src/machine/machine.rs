#![allow(non_snake_case)]

use super::{MOS6502Memory, MachineConfig, Memory};
use crate::mos6510::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6510,
};
use std::num::Wrapping;

pub struct MachineEvents {
    pub on_next: Option<fn(&mut Machine, &u64)>,
}

pub struct Machine {
    pub config: MachineConfig,
    pub cpu: MOS6510,
    pub mem: Box<dyn Memory + Send>,
    pub events: MachineEvents,
}

pub fn machine_loop(machine: &mut Machine) {
    let mut cycles = 0u64;
    loop {
        if let Some(max_cycles) = machine.config.max_cycles {
            if cycles > max_cycles {
                break;
            }
        }
        if !machine.next() {
            break;
        };
        if let Some(on_next) = machine.events.on_next {
            on_next(machine, &cycles);
        }
        if let Some(addr) = machine.config.exit_on_addr {
            if machine.PC() == addr {
                break;
            }
        }
        cycles += 1;
    }
}

pub trait RegSetter<T> {
    fn set_A(self, val: T);
    fn set_X(self, val: T);
    fn set_Y(self, val: T);
    fn set_SC(self, val: T);
}

impl RegSetter<u8> for &mut Machine {
    fn set_A(self, val: u8) {
        self.cpu.registers.accumulator = Wrapping(val);
    }
    fn set_X(self, val: u8) {
        self.cpu.registers.x = Wrapping(val);
    }
    fn set_Y(self, val: u8) {
        self.cpu.registers.y = Wrapping(val);
    }
    fn set_SC(self, val: u8) {
        self.cpu.registers.stack = Wrapping(val);
    }
}

impl RegSetter<Wrapping<u8>> for &mut Machine {
    fn set_A(self, val: Wrapping<u8>) {
        self.cpu.registers.accumulator = val;
    }
    fn set_X(self, val: Wrapping<u8>) {
        self.cpu.registers.x = val;
    }
    fn set_Y(self, val: Wrapping<u8>) {
        self.cpu.registers.y = val;
    }
    fn set_SC(self, val: Wrapping<u8>) {
        self.cpu.registers.stack = val;
    }
}

impl AsRef<Machine> for Machine {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Machine {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        Machine {
            config: config,
            cpu: MOS6510::new(),
            mem: Box::new(MOS6502Memory::new(size)),
            events: MachineEvents { on_next: None },
        }
    }

    // registry shortcuts
    pub fn A(&self) -> Wrapping<u8> {
        self.cpu.registers.accumulator
    }
    pub fn X(&self) -> Wrapping<u8> {
        self.cpu.registers.x
    }
    pub fn Y(&self) -> Wrapping<u8> {
        self.cpu.registers.y
    }
    pub fn A8(&self) -> u8 {
        self.cpu.registers.accumulator.0
    }
    pub fn X8(&self) -> u8 {
        self.cpu.registers.x.0
    }
    pub fn Y8(&self) -> u8 {
        self.cpu.registers.y.0
    }
    pub fn A16(&self) -> u16 {
        self.cpu.registers.accumulator.0 as u16
    }
    pub fn X16(&self) -> u16 {
        self.cpu.registers.x.0 as u16
    }
    pub fn Y16(&self) -> u16 {
        self.cpu.registers.y.0 as u16
    }
    pub fn P(&self) -> ProcessorStatus {
        self.cpu.registers.status
    }
    pub fn PC(&self) -> u16 {
        self.cpu.registers.counter
    }
    pub fn SC(&self) -> Wrapping<u8> {
        self.cpu.registers.stack
    }

    // boot sequence, etc
    pub fn power_on(&mut self) {
        // see https://www.pagetable.com/c64ref/c64mem/
        self.mem.set_byte(0x0000, 0x2f);
        self.mem.set_byte(0x0001, 0x37);

        // By default, after start, the PC is set to address from RST vector ($fffc)
        // http://wilsonminesco.com/6502primer/MemMapReqs.html
        self.cpu.registers.counter = self.mem.get_word(0xfffc);
    }

    pub fn start(&mut self) {
        let mut cycles = 0u64;
        loop {
            if let Some(max_cycles) = self.config.max_cycles {
                if cycles > max_cycles {
                    break;
                }
            }
            if !self.next() {
                break;
            };
            if let Some(addr) = self.config.exit_on_addr {
                if self.PC() == addr {
                    break;
                }
            }
            if let Some(on_next) = self.events.on_next {
                on_next(self, &cycles);
            }
            cycles += 1;
        }
    }

    pub fn next(&mut self) -> bool {
        let def = self.decode_op();
        let operand = self.decode_operand(&def);
        let address = if let Some(o) = &operand {
            self.decode_address(&def, &o)
        } else {
            None
        };
        let op = Operation::new(def, operand, address);
        (def.function)(&op, self);
        if self.config.disassemble {
            self.print_op(&op);
        }
        !(self.config.exit_on_brk && Mnemonic::BRK == def.mnemonic)
    }

    fn print_op(&self, op: &Operation) {
        let addr = self.PC() - op.def.len() as u16;
        let val = match op.def.len() {
            2 => format!("{:02x}   ", self.mem.get_byte(addr + 1)),
            3 => format!(
                "{:02x} {:02x}",
                self.mem.get_byte(addr + 1),
                self.mem.get_byte(addr + 2)
            ),
            _ => String::from("     "),
        };
        print!("{:04x}: {:02x} {} | {}", addr, op.def.opcode, val, op);
        if self.config.verbose {
            print!(
                "{}|  {}",
                " ".repeat(13 - op.to_string().len()),
                self.cpu.registers
            );
        }
        println!();
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
            None => panic!(
                "Opcode {:#04x} not found at address {:#06x}",
                opcode,
                self.PC() - 1
            ),
        }
    }

    fn decode_operand(&mut self, op: &OperationDef) -> Option<Operand> {
        match op.operand_len() {
            0 => None,
            1 => Some(Operand::Byte(self.get_byte_and_inc_pc())),
            2 => Some(Operand::Word(self.get_word_and_inc_pc())),
            _ => panic!("Invalid operand length"),
        }
    }

    // see http://www.emulator101.com/6502-addressing-modes.html
    // for indireact see JMP instruction on https://c64os.com/post/6502instructions
    fn decode_address(&self, op: &OperationDef, operand: &Operand) -> Option<u16> {
        let to_u16 = |a: u8, b: u8| -> (u16, u16) { (a as u16, b as u16) };
        match op.address_mode {
            AddressMode::Absolute => operand.get_word(),
            AddressMode::AbsoluteX => Some(operand.get_word().unwrap() + self.X16()),
            AddressMode::AbsoluteY => Some(operand.get_word().unwrap() + self.Y16()),
            AddressMode::ZeroPage => Some(operand.get_byte_as_u16().unwrap()),
            AddressMode::ZeroPageX => {
                let (o, x) = to_u16(operand.get_byte().unwrap(), self.X8());
                Some((o + x) & 0x00ff)
            }
            AddressMode::ZeroPageY => {
                let (o, y) = to_u16(operand.get_byte().unwrap(), self.Y8());
                Some((o + y) & 0x00ff)
            }
            AddressMode::Indirect => {
                let addr = operand.get_word().unwrap();
                let addr2 = (addr & 0xff00) | ((addr + 1) & 0x00ff); // page change not allowed!
                let (lo, hi) = to_u16(self.mem.get_byte(addr), self.mem.get_byte(addr2));
                Some(lo | hi << 8)
            }
            AddressMode::IndirectX => {
                let (o, x) = to_u16(operand.get_byte().unwrap(), self.X8());
                let lo = self.mem.get_byte((o + x) & 0x00ff) as u16;
                let hi = u16::from(self.mem.get_byte((o + x + 1) & 0x00ff)) << 8;
                Some(hi | lo)
            }
            AddressMode::IndirectY => {
                let (o, y) = to_u16(operand.get_byte().unwrap(), self.Y8());
                let lo = self.mem.get_byte(o) as u16;
                let hi = u16::from(self.mem.get_byte((o + 1) & 0x00ff)) << 8;
                Some((hi | lo) + y)
            }
            AddressMode::Relative => {
                //  TODO verify that - o must be signed int (check notation)
                let (o, pc) = (operand.get_byte().unwrap() as i8, self.PC() as i64);
                Some(((pc + o as i64) & 0xffff) as u16)
            }
            _ => None,
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

    // utility functions

    /// Returns current stack memory address
    pub fn stack_addr(&self) -> u16 {
        0x0100 | self.SC().0 as u16
    }

    // see https://en.wikipedia.org/wiki/Interrupts_in_65xx_processors
    fn handle_interrupt(&mut self, addr: u16) {
        let [msb, lsb] = self.PC().to_be_bytes();
        self.push(msb);
        self.push(lsb);
        self.push(u8::from(&self.P()));
        self.cpu.registers.status.interrupt_disable = true;
        self.cpu.registers.counter = self.mem.get_word(addr);
    }

    pub fn irq(&mut self) {
        if !self.P().interrupt_disable {
            self.handle_interrupt(0xfffe);
        }
    }

    pub fn nmi(&mut self) {
        self.handle_interrupt(0xfffa);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
