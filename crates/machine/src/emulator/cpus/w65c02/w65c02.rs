use std::{rc::Rc, cell::RefCell};
use genawaiter::{rc::Gen, Generator};

use crate::emulator::abstractions::{Addr, CPU, Addressable, IPin};
use crate::mos6502::steppers::{ OpGen, nop };

use super::W65C02_Pins;
// use genawaiter::{rc::gen, rc::Gen, yield_};

pub struct W65C02<'a> {
    pub pins: Rc<W65C02_Pins>,
    logic: W65C02Logic<'a>,
}

impl W65C02<'_> {
    pub fn new() -> Self {
        let pins = Rc::new(W65C02_Pins::new());
        let logic = W65C02Logic::new(Rc::clone(&pins));
        W65C02 { pins, logic }
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    /// Stores currently processed instruction. Can't be set by any operation.
    pub ir: u8,

    // actual registers
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8, // stack pointer
    pub s: u8,
}

pub struct W65C02Logic<'a> {
    reg: Registers,
    instruction_cycle: u8,
    pins: Rc<W65C02_Pins>,
    stepper: OpGen<'a>
}

impl W65C02Logic<'_> {
    pub fn new(pins: Rc<W65C02_Pins>) -> Self {
        W65C02Logic {
            reg: Registers::default(),
            instruction_cycle: 0,
            pins,
            stepper: nop()
        }
    }

    pub fn tick(&mut self) {
        self.stepper.resume();
        // let op: Gen<(), bool, _> = gen!({
        //     let opcode = self.read_byte(self.reg.pc);
        //     self.reg.pc = self.reg.pc.wrapping_add(1);
        //     yield_!();
        //     let arg_lo = self.read_byte(self.reg.pc);
        //     false
        // });
    }
}

impl CPU for W65C02Logic<'_> {
    fn cycles(&self) -> crate::emulator::abstractions::CPUCycles {
        todo!()
    }

    fn advance_cycles(&mut self, cycles: u8) -> Result<(), crate::emulator::EmulatorError> {
        todo!()
    }

    fn read_byte(&self, addr: Addr) -> u8 {
        self.pins.by_name("RWB").unwrap().set_high();
        self.pins.addr.write(addr);
        self.pins.data.read()
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        self.pins.by_name("RWB").unwrap().set_low();
        self.pins.addr.write(addr);
        self.pins.data.write(val);
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
