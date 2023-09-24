#![allow(non_snake_case)]
use super::{Addr, MachineConfig, MachineEvents, Memory};
use crate::mos6502::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6502,
};
use std::num::Wrapping;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MachineStatus {
    Stopped,
    Running,
    Debug,
}

pub trait RegSetter<T> {
    fn set_A(&mut self, val: T);
    fn set_X(&mut self, val: T);
    fn set_Y(&mut self, val: T);
    fn set_SC(&mut self, val: T);
}

pub trait Machine: RegSetter<u8> + RegSetter<Wrapping<u8>> {
    fn memory(&self) -> &Box<dyn Memory + Send + 'static>;
    fn memory_mut(&mut self) -> &mut Box<dyn Memory + Send + 'static>;
    fn cpu(&self) -> &MOS6502;
    fn cpu_mut(&mut self) -> &mut MOS6502;
    fn get_config(&self) -> &MachineConfig;
    fn get_events(&self) -> &MachineEvents;
    fn get_status(&self) -> MachineStatus;
    fn set_status(&mut self, status: MachineStatus);

    // registry shortcuts
    fn A(&self) -> Wrapping<u8> {
        self.cpu().registers.accumulator
    }
    fn X(&self) -> Wrapping<u8> {
        self.cpu().registers.x
    }
    fn Y(&self) -> Wrapping<u8> {
        self.cpu().registers.y
    }
    fn A8(&self) -> u8 {
        self.cpu().registers.accumulator.0
    }
    fn X8(&self) -> u8 {
        self.cpu().registers.x.0
    }
    fn Y8(&self) -> u8 {
        self.cpu().registers.y.0
    }
    fn A16(&self) -> u16 {
        self.cpu().registers.accumulator.0 as u16
    }
    fn X16(&self) -> u16 {
        self.cpu().registers.x.0 as u16
    }
    fn Y16(&self) -> u16 {
        self.cpu().registers.y.0 as u16
    }
    fn P(&self) -> ProcessorStatus {
        self.cpu().registers.status
    }
    fn PC(&self) -> u16 {
        self.cpu().registers.counter
    }
    fn SC(&self) -> Wrapping<u8> {
        self.cpu().registers.stack
    }
    fn set_PC(&mut self, addr: u16) {
        self.cpu_mut().registers.counter = addr;
    }

    fn get_byte(&self, addr: Addr) -> u8 {
        self.memory().get_byte(addr)
    }

    fn set_byte(&mut self, addr: Addr, val: u8) {
        self.memory_mut().set_byte(addr, val);
    }

    /// Start only changes the machine's status (and setups memory_
    /// but it doesn't cycle the machine! Either self.next() must be called
    /// or (better), a client should be used instead
    fn start(&mut self) {
        let mut cycles = 0u128;
        // see https://www.pagetable.com/c64ref/c64mem/
        self.set_byte(0x0000, 0x2f);
        self.set_byte(0x0001, 0x37);

        // By default, after start, the PC is set to address from RST vector ($fffc)
        // http://wilsonminesco.com/6502primer/MemMapReqs.html
        self.set_PC(self.memory().get_word(0xfffc));
        self.set_status(MachineStatus::Running);
    }

    fn stop(&mut self) {
        self.set_status(MachineStatus::Stopped);
    }

    fn debug(&mut self) {
        self.set_status(MachineStatus::Debug);
    }

    fn resume(&mut self) {
        self.set_status(MachineStatus::Running);
    }

    fn reset(&mut self) {
        panic!("Not implemented yet :-)");
    }

    fn execute_operation(&mut self, op: &Operation) -> u8;

    fn next(&mut self) -> bool {
        let def = { self.decode_op() };
        let operand = { self.decode_operand(&def) };
        let address = operand
            .as_ref()
            .map_or(None, |o| self.decode_address(&def, &o));
        let op = Operation::new(def.clone(), operand, address);

        if self.get_config().disassemble {
            self.print_op(&op);
        }

        if self.get_config().exit_on_brk && matches!(def.mnemonic, Mnemonic::BRK) {
            self.stop();
            return false;
        }

        self.execute_operation(&op);
        true
    }

    fn print_op(&self, op: &Operation) {
        let addr = self.PC().wrapping_sub(op.def.len() as u16);
        let val = match op.def.len() {
            2 => format!("{:02x}   ", self.get_byte(addr + 1)),
            3 => format!(
                "{:02x} {:02x}",
                self.get_byte(addr + 1),
                self.get_byte(addr + 2)
            ),
            _ => String::from("     "),
        };
        print!("{:04x}: {:02x} {} | {}", addr, op.def.opcode, val, op);
        if self.get_config().verbose {
            print!(
                "{}|  {}",
                " ".repeat(13 - op.to_string().len()),
                self.cpu().registers,
                // self.get_vars()
            );
        }
        println!();
    }

    // fn get_vars(&self) -> String {
    //     let a = self.memory().get_word(0x0010);
    //     let b = self.memory().get_word(0x0012);
    //     let c = self.memory().get_word(0x0014);
    //     let mut s = String::new();
    //     write!(&mut s, "a={:04x}, b={:04x}, c={:04x}", a, b, c);
    //     s
    // }

    fn get_byte_and_inc_pc(&mut self) -> u8 {
        let val = self.get_byte(self.PC());
        self.inc_counter();
        val
    }

    fn get_word_and_inc_pc(&mut self) -> u16 {
        let val = self.memory().get_word(self.PC());
        self.inc_counter();
        self.inc_counter();
        val
    }

    fn inc_counter(&mut self) {
        self.set_PC(self.PC().wrapping_add(1));
    }

    fn decode_op(&mut self) -> OperationDef {
        let opcode = self.get_byte_and_inc_pc();
        match self.cpu().operations.get(&opcode) {
            Some(op) => op.clone(),
            None => panic!(
                "Opcode {:#04x} not found at address {:#06x}",
                opcode,
                self.PC().wrapping_sub(1)
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
                let (lo, hi) = to_u16(self.get_byte(addr), self.get_byte(addr2));
                Some(lo | hi << 8)
            }
            AddressMode::IndirectX => {
                let (o, x) = to_u16(operand.get_byte().unwrap(), self.X8());
                let lo = self.get_byte((o + x) & 0x00ff) as u16;
                let hi = u16::from(self.get_byte((o + x + 1) & 0x00ff)) << 8;
                Some(hi | lo)
            }
            AddressMode::IndirectY => {
                let (o, y) = to_u16(operand.get_byte().unwrap(), self.Y8());
                let lo = self.get_byte(o) as u16;
                let hi = u16::from(self.get_byte((o + 1) & 0x00ff)) << 8;
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

    fn push(&mut self, val: u8) {
        let sc = self.SC().0 as u16;
        self.set_byte(0x0100 | sc, val);
        self.cpu_mut().registers.stack -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.cpu_mut().registers.stack += 1;
        let sc = self.SC().0 as u16;
        self.get_byte(0x0100 | sc)
    }

    fn load(&mut self, progmem: &[u8], addr: u16) {
        self.memory_mut().write(addr, progmem);
    }

    fn run(&mut self, addr: u16) {
        self.cpu_mut().registers.counter = addr;
        self.start();
    }

    // utility functions

    /// Returns current stack memory address
    fn stack_addr(&self) -> u16 {
        0x0100 | self.SC().0 as u16
    }

    // see https://en.wikipedia.org/wiki/Interrupts_in_65xx_processors
    fn handle_interrupt(&mut self, addr: u16) {
        let [msb, lsb] = self.PC().to_be_bytes();
        self.push(msb);
        self.push(lsb);
        self.push(u8::from(&self.P()));
        self.cpu_mut().registers.status.interrupt_disable = true;
        self.set_PC(self.memory().get_word(addr));
    }

    fn irq(&mut self) {
        if !self.P().interrupt_disable {
            self.handle_interrupt(0xfffe);
        }
    }

    fn nmi(&mut self) {
        self.handle_interrupt(0xfffa);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
