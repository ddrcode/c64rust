#![allow(non_snake_case)]

use super::{VIC_II};
use crate::mos6510::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6510,
};
use crate::machine::{ MachineConfig, Machine, Memory, RegSetter };
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

    // boot sequence, etc
    pub fn power_on(&mut self) {
        // see https://www.pagetable.com/c64ref/c64mem/
        self.machine.mem.set_byte(0x0000, 0x2f);
        self.machine.mem.set_byte(0x0001, 0x37);
        self.machine.mem.set_word(0x0003, 0xb1aa);
        self.machine.mem.set_word(0x0005, 0xb391);

        // Location where BASIC program text is stored
        // https://www.pagetable.com/c64ref/c64mem/#002C
        self.machine.mem.set_word(0x002b, 0x0801);

        // Highest address available to BASIC
        // see https://www.pagetable.com/c64ref/c64mem/#0037
        self.machine.mem.set_word(0x0037, 0xa000);

        // graphics register
        // https://www.c64-wiki.com/wiki/53265
        self.machine.mem.set_byte(0xd011, 0b00011000);

        // https://stackoverflow.com/questions/18811244/waiting-for-a-change-on-d012-c64-assembler
        // https://codebase64.org/doku.php?id=base:double_irq_explained
        self.machine.mem.set_byte(0xd012, 0b11000001);

        // By default, after start, the PC is set to address from RST vector ($fffc)
        // http://wilsonminesco.com/6502primer/MemMapReqs.html
        self.machine.cpu.registers.counter = self.machine.mem.get_word(0xfffc);

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
        }
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
}

