use std::sync::{Arc, Mutex};

use machine::{emulator::abstractions::{Addressable, Addr,  DeviceTrait, ArrayMemory}, utils::lock};

use super::CIA1;

pub struct C64IO {
    pub cia1: Arc<Mutex<CIA1>>,
    // FIXME it's just a fallback. Remove when all devies will be in place
    pub ram: Arc<Mutex<ArrayMemory>>
}

impl Addressable for C64IO {
    // addr is relative to $D000
    fn read_byte(&self, addr: Addr) -> u8 {
        let addr = addr+0xd000;
        if (0xc00..0xc0f).contains(&addr) {
            return lock::<CIA1>(&self.cia1).read_byte(addr);
        }
        lock::<ArrayMemory>(&self.ram).read_byte(addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        let addr = addr+0xd000;
        if (0xc00..0xc0f).contains(&addr) {
            return lock::<CIA1>(&self.cia1).write_byte(addr, value);
        }
        lock::<ArrayMemory>(&self.ram).write_byte(addr, value);
    }

    fn address_width(&self) -> u16 {
        16
    }
}

impl DeviceTrait for C64IO {}
