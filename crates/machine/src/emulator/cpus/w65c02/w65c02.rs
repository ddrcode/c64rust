use corosensei::{Coroutine, CoroutineResult};
use std::{cell::RefCell, rc::Rc};

use crate::emulator::abstractions::{Addr, Addressable, CPUCycles, PinStateChange, CPU, Pin, Component};
use crate::emulator::cpus::mos6502::{get_stepper, nop, OperationDef, Stepper, OPERATIONS};

use super::W65C02_Pins;
// use genawaiter::{rc::gen, rc::Gen, yield_};

pub struct DummyCPU;
impl CPU for DummyCPU {
    fn cycles(&self) -> CPUCycles {
        0
    }

    fn advance_cycles(&mut self) {}

    fn read_byte(&self, _addr: Addr) -> u8 {
        0
    }

    fn write_byte(&mut self, _addr: Addr, _val: u8) {}

    fn execute(&mut self, _val: u8) -> u8 {
        0
    }

    fn pc(&self) -> Addr {
        0
    }

    fn inc_pc(&mut self) {}
}

pub struct W65C02 {
    pub pins: Rc<W65C02_Pins>,
    logic: Rc<RefCell<W65C02Logic>>,
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
                self.logic.borrow_mut().tick();
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

pub struct W65C02Logic {
    reg: Registers,
    pins: Rc<W65C02_Pins>,
    stepper: Option<Stepper>,
    cycles: CPUCycles,
    self_ref: Rc<RefCell<dyn CPU>>,
}

impl W65C02Logic {
    pub fn new(pins: Rc<W65C02_Pins>) -> Rc<RefCell<Self>> {
        let logic = W65C02Logic {
            reg: Registers::default(),
            pins,
            stepper: None,
            cycles: 0,
            self_ref: Rc::new(RefCell::new(DummyCPU)),
        };

        let rc = Rc::new(RefCell::new(logic));
        let clone = Rc::clone(&rc);
        (*rc.borrow_mut()).self_ref = clone;

        rc
    }

    pub fn tick(&mut self) {
        if self.stepper.is_none() {
            self.reg.ir = self.read_byte(self.pc());
            let op = self.decode_op(&self.reg.ir);
            self.stepper = get_stepper(&op);
        } else {
            let cpu = Rc::clone(&self.self_ref);
            match self.stepper.as_mut().unwrap().resume(cpu) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulator::{abstractions::Pin, cpus::mos6502::Stepper};

    fn create_stepper() -> Stepper {
        Coroutine::new(|yielder, _input| {
            for _ in 0..3 {
                yielder.suspend(());
            }
            false
        })
    }

    // #[test]
    // fn test_steps() {
    //     let cpu = W65C02::new();
    //     (*cpu.logic.borrow_mut()).stepper = Some(create_stepper());
    //
    //     assert_eq!(0, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(1, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(2, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(3, cpu.logic.borrow().cycles());
    // }

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

    // #[test]
    // fn test_with_real_stepper() {
    //     let cpu = W65C02::new();
    //     let opdef = OPERATIONS.get(&0xad).unwrap(); // LDA, absolute
    //     (*cpu.logic.borrow_mut()).stepper = get_stepper(opdef);
    //
    //     assert_eq!(0, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(1, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(2, cpu.logic.borrow().cycles());
    //     cpu.logic.borrow_mut().tick();
    //     assert_eq!(3, cpu.logic.borrow().cycles());
    // }
}
