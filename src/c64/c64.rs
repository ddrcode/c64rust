#![allow(non_snake_case)]

use super::VIC_II;
use crate::machine::{Machine, MachineConfig, Memory, RegSetter};
use crate::mos6510::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6510,
};
use std::num::Wrapping;

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
                cpu: MOS6510::new(),
                mem: Memory::new(size),
            },
            gpu: VIC_II {},
        }
    }
    pub fn power_on(&mut self) {
        self.machine.power_on();
    }

    pub fn start(&mut self) {
        let mut cycles = 0u64;
        loop {
            if let Some(max_cycles) = self.machine.config.max_cycles {
                if cycles > max_cycles {
                    break;
                }
            }
            // it simulates line drawing (to avoid infinite loop waiting for next line)
            self.machine.mem.set_byte(0xd012, (cycles % 255) as u8);
            if !self.machine.next() {
                break;
            };
            if let Some(addr) = self.machine.config.exit_on_addr {
                if self.machine.PC() == addr {
                    break;
                }
            }
            cycles += 1;

            if cycles==600000 {
                self.machine.mem.set_byte(0x0277, 65);
                self.machine.mem.set_byte(0x00c6, 1); // number of keys in the keyboard buffer
                self.machine.mem.set_byte(0xffe4, 22);
            }
        }
        println!("{} cycles completed", cycles);
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.machine.mem.write(addr, progmem);
    }

    pub fn print_screen(&self) {
        self.gpu.print_screen(&self.machine.mem);
    }

    pub fn init_rom(&mut self, data: &[u8]) {
        self.machine.mem.init_rom_at_addr(0xa000, &data[..8192]);
        self.machine.mem.init_rom_at_addr(0xe000, &data[8192..]);
    }

    pub fn get_screen_memory(&self) -> String {
        let mut chars = String::new();
        for i in 0x0400..0x07e8 {
            let sc = self.machine.mem.get_byte(i);
            let ch = VIC_II::to_ascii(sc);
            chars.push(ch);
        }
        chars
    }
}
