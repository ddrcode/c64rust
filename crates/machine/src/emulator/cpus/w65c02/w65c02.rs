use std::{rc::Rc, cell::RefCell};

use crate::emulator::abstractions::{Addr, CPU, Addressable};

use super::W65C02_Pins;
// use genawaiter::{rc::gen, rc::Gen, yield_};

pub struct W65C02<T: CPU> {
    pub pins: W65C02_Pins,
    logic: T,
}

impl<T: CPU> W65C02<T> {
    pub fn new(logic: T) -> Self {
        let pins = W65C02_Pins::new();
        W65C02 { pins, logic }
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    /// Stores currently processed instruction. Can't be set by any operation.
    pub ir: u8,
    /// "Fake" register used for emulator. It stores pointers for indirect
    /// addressing modes
    pub dp: u16, // data pointer

    // actual registers
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8, // stack pointer
    pub s: u8,
}

pub struct W65C02Logic {
    reg: Registers,
    instruction_cycle: u8,
    mem: Rc<dyn Addressable>
}

impl W65C02Logic {
    pub fn new(mem: Rc<dyn Addressable>) -> Self {
        W65C02Logic {
            reg: Registers::default(),
            instruction_cycle: 0,
            mem
        }
    }

    pub fn tick(&mut self) {
        // let op: Gen<(), bool, _> = gen!({
        //     let opcode = self.read_byte(self.reg.pc);
        //     self.reg.pc = self.reg.pc.wrapping_add(1);
        //     yield_!();
        //     let arg_lo = self.read_byte(self.reg.pc);
        //     false
        // });
    }
}

impl CPU for W65C02Logic {
    fn cycles(&self) -> crate::emulator::abstractions::CPUCycles {
        todo!()
    }

    fn advance_cycles(&mut self, cycles: u8) -> Result<(), crate::emulator::EmulatorError> {
        todo!()
    }

    fn read_byte(&self, addr: Addr) -> u8 {
        self.mem.read_byte(addr)
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        // self.mem.write_byte(addr, val);
    }

    fn execute(&mut self, val: u8) -> u8 {
        todo!()
    }

    fn pc(&self) -> Addr {
        self.reg.pc
    }

    fn inc_pc(&mut self) {
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }
}
