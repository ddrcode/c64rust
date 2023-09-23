use super::{Addr, Memory};

pub struct MOS6502Memory {
    ram: Box<[u8]>,
    rom: Box<[u8]>,
}

impl MOS6502Memory {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        // let size: usize = 1 << 16;
        MOS6502Memory {
            ram: vec![0u8; size].into_boxed_slice(),
            rom: vec![0u8; 1 + u16::MAX as usize].into_boxed_slice(),
        }
    }

    pub fn init_rom_at_addr(&mut self, addr: Addr, data: &[u8]) {
        let mut idx = addr as usize;
        for byte in data.iter() {
            self.rom[idx] = *byte;
            idx += 1;
        }
    }
}

impl Memory for MOS6502Memory {
    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    #[allow(unused_comparisons)]
    fn mem(&self, addr: Addr) -> &[u8] {
        let flag = self.ram[1] & 0b00000111;
        if flag & 1 > 0 && addr >= 0xa000 && addr <= 0xbfff {
            return &self.rom;
        };
        if flag & 2 > 0 && addr >= 0xe000 && addr <= 0xffff {
            return &self.rom;
        };
        &self.ram
    }

    fn init_rom(&mut self, data: &[u8]) {
        let addr: usize = 0x10000 - data.len();
        self.init_rom_at_addr(addr as u16, data);
    }

    fn set_byte(&mut self, addr: Addr, val: u8) {
        self.ram[addr as usize] = val;
    }

    fn set_word(&mut self, addr: Addr, val: u16) {
        let idx = addr as usize;
        let [high, low] = val.to_be_bytes();
        self.ram[idx] = low;
        self.ram[idx + 1] = high; // little endian!
    }
}
