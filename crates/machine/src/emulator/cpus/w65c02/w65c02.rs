use corosensei::CoroutineResult;
use std::{cell::RefCell, rc::Rc};

use crate::emulator::abstractions::{Addr, CPUCycles, Component, Pin, PinStateChange, Pins, CPU, AsAny};
use crate::emulator::cpus::mos6502::{get_stepper, nop, OperationDef, Stepper, OPERATIONS};
use crate::utils::bool_to_bit;

use super::W65C02_Pins;
// use genawaiter::{rc::gen, rc::Gen, yield_};

pub struct W65C02 {
    pub pins: Rc<W65C02_Pins>,
    logic: W65C02Logic,
}

impl W65C02 {
    pub fn new() -> Self {
        let pins = Rc::new(W65C02_Pins::new());
        let logic = W65C02Logic::new(Rc::clone(&pins));
        W65C02 { pins, logic }

        // cpu.pins
        //     .by_name("PHI2")
        //     .unwrap()
        // .set_handler(Rc::clone(&cpu) as Rc<dyn PinStateChange>)
        // .unwrap();
    }
}

impl Component for W65C02 {
    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.by_name(name)
    }
}

impl PinStateChange for W65C02 {
    fn on_state_change(&mut self, pin: &Pin) {
        match &*pin.name() {
            "PHI2" => {
                self.logic.tick();
            }
            _ => {}
        };
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

pub struct CpuState {
    pub reg: Registers,
    pub pins: Rc<W65C02_Pins>,
}

impl CpuState {
    pub fn read_byte(&self, addr: Addr) -> u8 {
        self.pins.by_name("RW").unwrap().set_high().unwrap();
        self.pins.addr.write(addr);
        self.pins.data.read()
    }

    pub fn write_byte(&mut self, addr: Addr, val: u8) {
        self.pins.by_name("RW").unwrap().set_low().unwrap();
        self.pins.addr.write(addr);
        self.pins.data.write(val);
    }

    pub fn inc_pc(&mut self) {
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    pub fn execute(&mut self, _val: u8) -> u8 {
        0
    }

    pub fn set_ir_from_pc(&mut self) -> u8 {
        self.reg.ir = self.read_byte(self.reg.pc);
        self.reg.ir
    }

    pub fn pc(&self) -> u16 {
        self.reg.pc
    }

    pub fn set_a(&mut self, val: u8) {
        self.reg.a = val;
    }

    pub fn set_x(&mut self, val: u8) {
        self.reg.x = val;
    }

    pub fn set_y(&mut self, val: u8) {
        self.reg.y = val;
    }

    pub fn set_flag_n(&mut self, val: bool) {
        self.reg.s = self.reg.s & 0b0111_1111 | bool_to_bit(&val, 7);
    }

    pub fn set_flag_z(&mut self, val: bool) {
        self.reg.s = self.reg.s & 0b1111_1110 | (val as u8);
    }
}

pub struct W65C02Logic {
    stepper: Option<Stepper>,
    cycles: CPUCycles,
    state: Rc<RefCell<CpuState>>,
}

impl W65C02Logic {
    pub fn new(pins: Rc<W65C02_Pins>) -> Self {
        let logic = W65C02Logic {
            state: Rc::new(RefCell::new(CpuState {
                reg: Registers::default(),
                pins,
            })),
            stepper: None,
            cycles: 0,
        };

        logic
    }

    pub fn tick(&mut self) {
        if self.stepper.is_none() {
            let ir = self.state.borrow_mut().set_ir_from_pc();
            let op = self.decode_op(&ir);
            self.stepper = get_stepper(&op);
        } else {
            // let cpu = Rc::clone(&self.self_ref);
            let v = self
                .stepper
                .as_mut()
                .unwrap()
                .resume(Rc::clone(&self.state));
            match v {
                CoroutineResult::Yield(()) => {}
                CoroutineResult::Return(_) => {
                    self.stepper = None;
                }
            }
        }
        self.advance_cycles();
    }

    fn decode_op(&self, opcode: &u8) -> OperationDef {
        match OPERATIONS.get(&opcode) {
            Some(op) => op.clone(),
            None => panic!(
                "Opcode {:#04x} not found at address {:#06x}",
                opcode,
                self.pc()
            ),
        }
    }
}

impl CPU for W65C02Logic {
    fn cycles(&self) -> CPUCycles {
        self.cycles
    }

    fn advance_cycles(&mut self) {
        self.cycles = self.cycles.wrapping_add(1);
    }

    fn read_byte(&self, addr: Addr) -> u8 {
        self.state.borrow().read_byte(addr)
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        self.state.borrow_mut().write_byte(addr, val);
    }

    fn execute(&mut self, _val: u8) -> u8 {
        todo!()
    }

    fn pc(&self) -> Addr {
        self.state.borrow().reg.pc
    }

    fn inc_pc(&mut self) {
        self.state.borrow_mut().inc_pc();
    }
}

impl AsAny for W65C02 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulator::{abstractions::Pin, cpus::mos6502::Stepper};

    fn create_stepper() -> Stepper {
        corosensei::Coroutine::new(|yielder, _input| {
            for _ in 0..3 {
                yielder.suspend(());
            }
            false
        })
    }

    #[test]
    fn test_steps() {
        let mut cpu = W65C02::new();
        cpu.logic.stepper = Some(create_stepper());

        assert_eq!(0, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(1, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(2, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(3, cpu.logic.cycles());
    }

    // #[test]
    // fn test_steps_with_clock_signal() {
    //     let clock = Pin::output();
    //     let cpu = W65C02::new();
    //     (*cpu.logic.borrow_mut()).stepper = Some(create_stepper());
    //     Pin::link(&clock, &cpu.pins.by_name("PHI2").unwrap()).unwrap();
    //
    //     assert_eq!(0, cpu.logic.borrow().cycles());
    //     clock.toggle();
    //     assert_eq!(1, cpu.logic.borrow().cycles());
    //     clock.toggle();
    //     assert_eq!(2, cpu.logic.borrow().cycles());
    //     clock.toggle();
    //     assert_eq!(3, cpu.logic.borrow().cycles());
    // }

    #[test]
    fn test_with_real_stepper() {
        let mut cpu = W65C02::new();
        let opdef = OPERATIONS.get(&0xad).unwrap(); // LDA, absolute
        cpu.logic.stepper = get_stepper(opdef);

        assert_eq!(0, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(1, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(2, cpu.logic.cycles());
        cpu.logic.tick();
        assert_eq!(3, cpu.logic.cycles());
    }
}
