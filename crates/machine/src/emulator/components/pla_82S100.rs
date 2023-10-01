// 00	0	0	0	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 01	0	0	0	0	1	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 02	0	0	0	1	0	RAM	RAM	RAM	CHI	RAM	CHR	KRN
// 03	0	0	0	1	1	RAM	RAM	CLO	CHI	RAM	CHR	KRN
// 04	0	0	1	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 05	0	0	1	0	1	RAM	RAM	RAM	RAM	RAM	I/O	RAM
// 06	0	0	1	1	0	RAM	RAM	RAM	CHI	RAM	I/O	KRN
// 07	0	0	1	1	1	RAM	RAM	CLO	CHI	RAM	I/O	KRN
// 08	0	1	0	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 09	0	1	0	0	1	RAM	RAM	RAM	RAM	RAM	CHR	RAM
// 10	0	1	0	1	0	RAM	RAM	RAM	RAM	RAM	CHR	KRN
// 11	0	1	0	1	1	RAM	RAM	CLO	BSC	RAM	CHR	KRN
// 12	0	1	1	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 13	0	1	1	0	1	RAM	RAM	RAM	RAM	RAM	I/O	RAM
// 14	0	1	1	1	0	RAM	RAM	RAM	RAM	RAM	I/O	KRN
// 15	0	1	1	1	1	RAM	RAM	CLO	BSC	RAM	I/O	KRN
// 16	1	0	0	0	0	RAM	-	CLO	-	-	I/O	CHI
// 17	1	0	0	0	1	RAM	-	CLO	-	-	I/O	CHI
// 18	1	0	0	1	0	RAM	-	CLO	-	-	I/O	CHI
// 19	1	0	0	1	1	RAM	-	CLO	-	-	I/O	CHI
// 20	1	0	1	0	0	RAM	-	CLO	-	-	I/O	CHI
// 21	1	0	1	0	1	RAM	-	CLO	-	-	I/O	CHI
// 22	1	0	1	1	0	RAM	-	CLO	-	-	I/O	CHI
// 23	1	0	1	1	1	RAM	-	CLO	-	-	I/O	CHI
// 24	1	1	0	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 25	1	1	0	0	1	RAM	RAM	RAM	RAM	RAM	CHR	RAM
// 26	1	1	0	1	0	RAM	RAM	RAM	RAM	RAM	CHR	KRN
// 27	1	1	0	1	1	RAM	RAM	RAM	BSC	RAM	CHR	KRN
// 28	1	1	1	0	0	RAM	RAM	RAM	RAM	RAM	RAM	RAM
// 29	1	1	1	0	1	RAM	RAM	RAM	RAM	RAM	I/O	RAM
// 30	1	1	1	1	0	RAM	RAM	RAM	RAM	RAM	I/O	KRN
// 31	1	1	1	1	1	RAM	RAM	RAM	BSC	RAM	I/O	KRN

use lazy_static;

use crate::{emulator::abstractions::{Addr, AddressResolver, Addressable}, utils::if_else};

lazy_static! {
    static ref BANKS: [[u8; 7]; 32] = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 3, 0, 5, 6],
        [0, 0, 1, 3, 0, 5, 6],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 3, 0, 4, 6],
        [0, 0, 1, 3, 0, 4, 6],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 5, 0],
        [0, 0, 0, 0, 0, 5, 6],
        [0, 0, 1, 2, 0, 5, 6],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 0, 4, 6],
        [0, 0, 1, 2, 0, 4, 6],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 7, 1, 7, 7, 4, 3],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 5, 0],
        [0, 0, 0, 0, 0, 5, 6],
        [0, 0, 0, 2, 0, 5, 6],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 0, 4, 6],
        [0, 0, 0, 2, 0, 4, 6],
    ];
}

/// It's a generic implementation (hence an array of Addressables),
/// but in C64 the structure is as follows
/// 0 - RAM
/// 1 - Cartirdge ROM (lo)
/// 2 - BASIC ROM
/// 3 - Cartridge ROM (hi)
/// 4 - I/O
/// 5 - CHAR ROM
/// 6 - KERNAL
/// 7 - invalid

pub struct PLA_82S100<'a> {
    devices: Box<[&'a mut dyn Addressable; 7]>,
    pin8: bool, // GAME on C64
    pin9: bool,
}

impl<'a> Addressable for PLA_82S100<'a> {
    fn read_byte(&self, addr: Addr) -> u8 {
        let id = self.get_device_id(addr);
        if id == 7 { return 0; } // TODO check what to do for this case
        self.devices[id as usize].read_byte(addr)
    }
    fn write_byte(&mut self, addr: Addr, value: u8) {
        let id = self.get_device_id(addr);
        let real_id = if_else(id==4, 4, 0); // if not i/o, write to ram
        self.devices[real_id as usize].write_byte(addr, value);
    }

    fn writeable() -> bool
    where
        Self: Sized,
    {
        true
    }
}

impl<'a> AddressResolver for PLA_82S100<'a> {}

impl<'a> PLA_82S100<'a> {
    fn get_device_id(&self, addr: Addr) -> u8 {
        let mut flag = self.devices[0].read_byte(0x0001);
        flag = (flag & 0b111) | (u8::from(self.pin8) << 4) | (u8::from(self.pin9) << 5);
        let bank = &BANKS[flag as usize];
        match addr {
            0x0000..=0x0fff => bank[0],
            0x1000..=0x7fff => bank[1],
            0x8000..=0x9fff => bank[2],
            0xa000..=0xbfff => bank[3],
            0xc000..=0xcfff => bank[4],
            0xd000..=0xdfff => bank[5],
            0xe000..=0xffff => bank[6],
        }
    }
}
