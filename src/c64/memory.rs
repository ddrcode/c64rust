use crate::machine::{Addr, Memory};
use std::rc::Rc;

// TODO consider better way of initializing the memory
// see this: https://www.reddit.com/r/rust/comments/jzwwqb/about_creating_a_boxed_slice/
// and this: https://www.reddit.com/r/rust/comments/c4zdue/newbie_question_array_in_a_struct/

/// Emulation of C64 memory.
/// C64 uses 16-bit addressing, but it provides more memory than can be addressed with u16:
/// 64kB RAM, ~20kB ROM, plus - optionally - extra ROM when a cartrige is used.
/// To solve that the flags at 0x00 define memory access model, and depending on its value
/// the read operation points to various memory types (write operations always point to RAM).
/// Here is a simplified memory map (based on https://www.c64-wiki.com/wiki/Memory_Map):
/// $8000-$9FFF: Cartridge ROM
/// $A000-$BFFF: BASIC interpreter (C64 ROM) or Cartridge ROM
/// $D000-$DFFF: Character generator ROM
/// $E000-$FFFF: Kernal (C64 ROM) or Cartridge ROM
/// The emulator provides 64kB of RAM and 64kB of ROM, but no extra memory for
/// cartridges - it simply overrides ROM for cartridges (TBC whether such simplification
/// is sufficient).
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

    fn is_io(&self, addr: Addr) -> bool {
        let r1 = self.ram[1];
        r1 & 100 > 1 && r1 & 11 > 1 && addr >= 0xd000 && addr <= 0xdfff
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
        let len = data.len();
        if len == 16384 {
            // the size of original rom
            self.init_rom_at_addr(0xa000, &data[..8192]);
            self.init_rom_at_addr(0xe000, &data[8192..]);
        } else {
            // custom rom
            let addr: usize = 0x10000 - len;
            self.init_rom_at_addr(addr as u16, data);
        }
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
