use crate::emulator::abstractions::Addr;

use super::W65C02_Pins;
// use genawaiter::{rc::gen, rc::Gen, yield_};

pub struct W65C02 {
    pub pins: W65C02_Pins,
    logic: W65C02Logic,
}

impl W65C02 {
    pub fn new() -> Self {
        let pins = W65C02_Pins::new();
        let logic = W65C02Logic::new();
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
}

impl W65C02Logic {
    pub fn new() -> Self {
        W65C02Logic {
            reg: Registers::default(),
            instruction_cycle: 0,
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

    pub fn read_byte(&self, addr: Addr) -> u8 {
        0
    }
}
