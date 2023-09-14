use crate::machine::Memory;

type Addr = u16;

pub struct C64Memory {
    ram: Box<[u8]>,
    rom: Box<[u8]>,
}

impl C64Memory {
    pub fn new(size: usize) -> Self {
        // let size: usize = 1 << 16;
        C64Memory {
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

impl Memory for C64Memory {
    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    fn mem(&self, addr: Addr) -> &[u8] {
        let flag = self.ram[1] & 0b00000111;
        if flag & 1 > 0 && addr >= 0xa000 && addr <= 0xbfff {
            return &self.rom;
        };
        if flag & 2 > 0 && addr >= 0xe000 && addr <= 0xffff {
            return &self.rom;
        };
        if flag & 4 == 0 && addr >= 0xd000 && addr <= 0xdfff {
            return &self.rom;
        };
        &self.ram
    }

    fn init_rom(&mut self, data: &[u8]) {
        self.init_rom_at_addr(0xa000, &data[..8192]);
        self.init_rom_at_addr(0xe000, &data[8192..]);
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
