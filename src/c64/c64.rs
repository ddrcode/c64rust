#![allow(non_snake_case)]

use super::{C64Memory, VIC_II};
use crate::machine::{impl_reg_setter, Machine, MachineConfig, MachineEvents, Memory, RegSetter};
use crate::mos6510::{execute_operation, Operation, MOS6510};
use std::num::Wrapping;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub struct C64 {
    config: MachineConfig,
    mos6510: MOS6510,
    mem: Box<dyn Memory + Send>,
    events: MachineEvents,
    gpu: VIC_II,
}

impl C64 {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        C64 {
            config: config,
            mos6510: MOS6510::new(),
            mem: Box::new(C64Memory::new(size)),
            events: MachineEvents {
                on_next: Some(|machine, cycle| {
                    // it simulates line drawing (to avoid infinite loop waiting for next line)
                    machine.memory_mut().set_byte(0xd012, (*cycle % 255) as u8);
                }),
            },
            gpu: VIC_II {},
        }
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.memory_mut().write(addr, progmem);
    }

    pub fn print_screen(&self) {
        self.gpu.print_screen(&self.memory());
    }

    pub fn get_screen_memory(&self) -> String {
        let mut chars = String::new();
        for i in 0x0400..0x07e8 {
            let sc = self.memory().get_byte(i);
            let ch = VIC_II::to_ascii(sc);
            chars.push(ch);
        }
        chars
    }

    pub fn send_key(&mut self, ch: char) {
        let sc = VIC_II::ascii_to_petscii(ch);
        self.memory_mut().set_byte(0x0277, sc);
        self.memory_mut().set_byte(0x00c6, 1); // number of keys in the keyboard buffer
        self.memory_mut().set_byte(0xffe4, 22);
        // self.machine.memory_mut().set_byte(0xc5, 2);
        // self.machine.memory_mut().set_byte(0xcb, 3);
    }
}

impl_reg_setter!(C64);

impl Machine for C64 {
    fn memory(&self) -> &Box<dyn Memory + Send + 'static> {
        &self.mem
    }

    fn memory_mut(&mut self) -> &mut Box<dyn Memory + Send + 'static> {
        &mut self.mem
    }

    fn cpu(&self) -> &MOS6510 {
        &self.mos6510
    }

    fn cpu_mut(&mut self) -> &mut MOS6510 {
        &mut self.mos6510
    }

    fn get_config(&self) -> &MachineConfig {
        &self.config
    }

    fn get_events(&self) -> &MachineEvents {
        &self.events
    }

    fn execute_operation(&mut self, op: &Operation) -> u8 {
        execute_operation(&op, self)
    }
}
