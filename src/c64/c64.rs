#![allow(non_snake_case)]

use super::VIC_II;
use crate::machine::{Machine, MachineConfig, Memory, RegSetter, MachineEvents};
use crate::mos6510::{
    AddressMode, Mnemonic, Operand, Operation, OperationDef, ProcessorStatus, MOS6510,
};
use std::num::Wrapping;
use std::sync::{ Arc, Mutex };
use std::time;
use std::thread;

pub struct C64 {
    pub machine: Machine,
    pub gpu: VIC_II,
}

pub fn machine_loop(c64mutex: Arc<Mutex<C64>>) {
    let mut cycles = 0u64;
    let ten_millis = time::Duration::from_nanos(1);
    loop {
        {
            let mut c64 = c64mutex.lock().unwrap();
            if let Some(max_cycles) = c64.machine.config.max_cycles {
                if cycles > max_cycles {
                    break;
                }
            }
            if !c64.machine.next() {
                break;
            };
            if let Some(on_next) = c64.machine.events.on_next {
                on_next(&mut c64.machine, &cycles);
            }
            if let Some(addr) = c64.machine.config.exit_on_addr {
                if c64.machine.PC() == addr {
                    break;
                }
            }
        }
        // thread::sleep(ten_millis);
        cycles += 1;
    }
}

impl C64 {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        C64 {
            machine: Machine {
                config: config,
                cpu: MOS6510::new(),
                mem: Memory::new(size),
                events: MachineEvents {
                    on_next: Some(|machine, cycles| {
                        // it simulates line drawing (to avoid infinite loop waiting for next line)
                        machine.mem.set_byte(0xd012, (*cycles % 255) as u8);

                        if *cycles==600000 {
                            machine.mem.set_byte(0x0277, 65);
                            machine.mem.set_byte(0x00c6, 1); // number of keys in the keyboard buffer
                            machine.mem.set_byte(0xffe4, 22);
                        }
                    })
                }
            },
            gpu: VIC_II {},
        }
    }
    pub fn power_on(&mut self) {
        self.machine.power_on();
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
