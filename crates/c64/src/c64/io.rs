use std::sync::{Arc, Mutex};

use machine::{
    emulator::abstractions::{Addr, Addressable, ArrayMemory, DeviceTrait},
    utils::lock,
};

use super::{CIA1, CIA2};

pub struct C64IO {
    pub cia1: Arc<Mutex<CIA1>>,
    pub cia2: Arc<Mutex<CIA2>>,
    // FIXME RAM is just a fallback. Remove when all devies will be in place
    pub ram: Arc<Mutex<ArrayMemory>>,
}

impl Addressable for C64IO {
    // addr is relative to $D000
    fn read_byte(&self, addr: Addr) -> u8 {
        // The CIA registers are mirrored each 16 bytes
        // so 0xdc10 is same as 0xdc00
        if (0xc00..=0xcff).contains(&addr) {
            lock::<CIA1>(&self.cia1).read_byte((addr - 0xc00) & 0x000f)
        } else if (0xd00..=0xdff).contains(&addr) {
            lock::<CIA2>(&self.cia2).read_byte((addr - 0xd00) & 0x000f)
        } else {
            lock::<ArrayMemory>(&self.ram).read_byte(addr + 0xd000)
        }
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        if (0xc00..=0xc0f).contains(&addr) {
            return lock::<CIA1>(&self.cia1).write_byte(addr - 0xc00, value);
        }
        lock::<ArrayMemory>(&self.ram).write_byte(addr+0xd000, value);
    }

    fn address_width(&self) -> u16 {
        16
    }
}

impl DeviceTrait for C64IO {}
