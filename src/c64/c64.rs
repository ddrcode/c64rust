#![allow(non_snake_case)]

use super::{C64Memory, VIC_II};
use crate::machine::{Machine, MachineConfig, MachineEvents, Memory, RegSetter};
use crate::mos6510::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6510,
};
use std::num::Wrapping;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub struct C64 {
    pub machine: Machine,
    pub gpu: VIC_II,
}

impl C64 {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        C64 {
            machine: Machine {
                config: config,
                mos6510: MOS6510::new(),
                mem: Box::new(C64Memory::new(size)),
                events: MachineEvents {
                    on_next: Some(|machine, cycle| {
                        // it simulates line drawing (to avoid infinite loop waiting for next line)
                        machine.memory_mut().set_byte(0xd012, (*cycle % 255) as u8);
                    }),
                },
            },
            gpu: VIC_II {},
        }
    }
    pub fn power_on(&mut self) {
        self.machine.power_on();
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.machine.memory_mut().write(addr, progmem);
    }

    pub fn print_screen(&self) {
        self.gpu.print_screen(&self.machine.memory());
    }

    pub fn get_screen_memory(&self) -> String {
        let mut chars = String::new();
        for i in 0x0400..0x07e8 {
            let sc = self.machine.memory().get_byte(i);
            let ch = VIC_II::to_ascii(sc);
            chars.push(ch);
        }
        chars
    }

    pub fn send_key(&mut self, ch: char) {
        let sc = VIC_II::ascii_to_petscii(ch);
        self.machine.memory_mut().set_byte(0x0277, sc);
        self.machine.memory_mut().set_byte(0x00c6, 1); // number of keys in the keyboard buffer
        self.machine.memory_mut().set_byte(0xffe4, 22);
        // self.machine.memory_mut().set_byte(0xc5, 2);
        // self.machine.memory_mut().set_byte(0xcb, 3);
    }
}

impl AsRef<Machine> for C64 {
    fn as_ref(&self) -> &Machine {
        &self.machine
    }
}

impl AsMut<Machine> for C64 {
    fn as_mut(&mut self) -> &mut Machine {
        &mut self.machine
    }
}
