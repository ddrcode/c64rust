type Addr = u16;

pub struct Memory {
    ram: Box<[u8]>,
    rom: Box<[u8]>
}

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
impl Memory {
    pub fn new(rom: Option<&[u8]>) -> Self {
        let size: usize = 1 << 16;
        Memory {
            ram: vec![0u8; size].into_boxed_slice(),
            rom: vec![0u8; size].into_boxed_slice()
        }
    }

    pub fn init_rom(&mut self, data: &[u8]) {
        let mut idx = 0xa000 as usize;
        for byte in data.iter() {
            self.rom[idx] = *byte;
            idx += 1;
            if idx == 0xc000 { idx = 0xe000 };
        }
    }

    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    fn mem(&self, addr: Addr) -> &[u8] {
        let flag = self.ram[1] & 0b00000111;
        if flag & 1 > 0 && addr >= 0xa000 && addr <= 0xbfff { return &self.rom };
        if flag & 2 > 0 && addr >= 0xe000 && addr <= 0xffff { return &self.rom };
        if flag & 4 == 0 && addr >= 0xd000 && addr <= 0xdfff { return &self.rom };
        &self.ram
    }

    pub fn get_byte(&self, addr: Addr) -> u8 {
        self.mem(addr)[addr as usize]
    }

    pub fn set_byte(&mut self, addr: Addr, val: u8) {
        self.ram[addr as usize] = val;
    }

    pub fn get_word(&self, addr: Addr) -> u16 {
        let idx = addr as usize;
        let mem = self.mem(addr);
        (mem[idx] as u16) | ((mem[idx+1] as u16) << 8)

    }

    pub fn set_word(&mut self, addr: Addr, val: u16) {
        let idx = addr as usize;
        let [high, low] = val.to_be_bytes();
        self.ram[idx] = low;
        self.ram[idx+1] = high; // little endian!
    }

    pub fn write(&mut self, addr: Addr, data: &[u8]) {
        let mut idx = addr as usize;
        for byte in data.iter() {
            self.ram[idx] = *byte;
            idx += 1;
        }
    }
}
