use super::{
    cia6526::{CIA1, CIA2},
    io::C64IO,
};
use machine::{
    emulator::{
        abstractions::{Accessor, AddressResolver, Addressable, ArrayMemory, Device},
        components::PLA_82S100,
    },
    Addr, Memory,
};

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

// pub struct C64Memory {
//     ram: Box<[u8]>,
//     rom: Box<[u8]>,
// }
//
pub struct C64Memory {
    pla: PLA_82S100,
}
impl C64Memory {
    pub fn new(cia1: &Device<CIA1>, cia2: &Device<CIA2>) -> Self {
        let mut pla = PLA_82S100::default();
        let ram = Device::from(ArrayMemory::new(0xffff, 16));
        pla.link_ram(ram.mutex());

        let io = Device::from(C64IO {
            ram: ram.mutex(),
            cia1: cia1.mutex(),
            cia2: cia2.mutex(),
        });

        // FIXME careful - there is hardcoded address inside the PLA
        pla.link_io(io.mutex());

        C64Memory { pla }
    }
}

impl Memory for C64Memory {
    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    #[allow(unused_comparisons)]
    fn mem(&self, _addr: Addr) -> &[u8] {
        panic!("shuldnt use");
    }

    fn init_rom(&mut self, data: &[u8]) {
        let len = data.len();
        if len == 16384 {
            // the size of original rom
            self.pla
                .link_basic(Device::from(ArrayMemory::from_data(&data[..8192], 16)).mutex());
            self.pla
                .link_kernal(Device::from(ArrayMemory::from_data(&data[8192..], 16)).mutex());
        } else {
            // custom rom
            let addr: usize = 0x10000 - len;
            //self.init_rom_at_addr(addr as u16, data);
            self.pla
                .link_kernal(Device::from(ArrayMemory::from_data(&data, 16)).mutex());
        }
    }

    fn init_rom_at_addr(&mut self, _addr: Addr, data: &[u8]) {
        self.pla
            .link_chargen(Device::from(ArrayMemory::from_data(&data, 16)).mutex());
    }
    fn write_byte(&mut self, addr: Addr, val: u8) {
        self.pla.write_byte(addr, val);
    }
    fn read_byte(&self, addr: Addr) -> u8 {
        self.pla.read_byte(addr)
    }

    fn fragment(&self, from: Addr, to: Addr) -> Vec<u8> {
        let mut vec = Vec::<u8>::with_capacity((to - from) as usize);
        let range = std::ops::Range {
            start: from,
            end: to,
        };
        for i in range {
            vec.push(self.read_byte(i));
        }
        vec
    }

    fn size(&self) -> usize {
        panic!("shuldnt use");
    }
    fn read_word(&self, addr: Addr) -> u16 {
        self.pla.read_word(addr)
    }
}
