use std::sync::{Arc, Mutex};

use machine::{
    emulator::abstractions::{Addr, Addressable, ArrayMemory, DeviceTrait},
    utils::lock,
};

use super::{CIA1, CIA2};

pub struct C64IO {
    pub cia1: Arc<Mutex<CIA1>>,
    pub cia2: Arc<Mutex<CIA2>>,
    // FIXME it's just a fallback. Remove when all devies will be in place
    pub ram: Arc<Mutex<ArrayMemory>>,
}

impl Addressable for C64IO {
    // addr is relative to $D000
    fn read_byte(&self, addr: Addr) -> u8 {
        let addr = addr + 0xd000;
        if (0xdc00..=0xdc0f).contains(&addr) {
            lock::<CIA1>(&self.cia1).read_byte(addr)
        } else if (0xdd00..=0xdd0f).contains(&addr) {
            lock::<CIA2>(&self.cia2).read_byte(addr)
        } else {
            lock::<ArrayMemory>(&self.ram).read_byte(addr)
        }
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        let addr = addr + 0xd000;
        if (0xdc00..=0xdc0f).contains(&addr) {
            return lock::<CIA1>(&self.cia1).write_byte(addr, value);
        }
        lock::<ArrayMemory>(&self.ram).write_byte(addr, value);
    }

    fn address_width(&self) -> u16 {
        16
    }
}

impl DeviceTrait for C64IO {}
