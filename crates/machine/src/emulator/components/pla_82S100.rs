use crate::utils::lock;
use lazy_static;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    emulator::abstractions::{Addr, AddressResolver, Addressable},
    utils::if_else,
};

lazy_static! {

    /// Modified version of a table from
    /// [Bank switching](https:///www.c64-wiki.com/wiki/Bank_Switching)
    ///
    /// ```text
    /// MODE         B0  B1  B2  B3  B4  B5  B6
    /// -----------------------------------------
    /// 00	00000	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 01	00001	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 02	00010	RAM	RAM	RAM	CHI	RAM	CHR	KRN
    /// 03	00011	RAM	RAM	CLO	CHI	RAM	CHR	KRN
    /// 04	00100	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 05	00101	RAM	RAM	RAM	RAM	RAM	I/O	RAM
    /// 06	00110	RAM	RAM	RAM	CHI	RAM	I/O	KRN
    /// 07	00111	RAM	RAM	CLO	CHI	RAM	I/O	KRN
    /// 08	01000	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 09	01001	RAM	RAM	RAM	RAM	RAM	CHR	RAM
    /// 10	01010	RAM	RAM	RAM	RAM	RAM	CHR	KRN
    /// 11	01011	RAM	RAM	CLO	BSC	RAM	CHR	KRN
    /// 12	01100	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 13	01101	RAM	RAM	RAM	RAM	RAM	I/O	RAM
    /// 14	01110	RAM	RAM	RAM	RAM	RAM	I/O	KRN
    /// 15	01111	RAM	RAM	CLO	BSC	RAM	I/O	KRN
    /// 16	10000	RAM	-	CLO	-	-	I/O	CHI
    /// 17	10001	RAM	-	CLO	-	-	I/O	CHI
    /// 18	10010	RAM	-	CLO	-	-	I/O	CHI
    /// 19	10011	RAM	-	CLO	-	-	I/O	CHI
    /// 20	10100	RAM	-	CLO	-	-	I/O	CHI
    /// 21	10101	RAM	-	CLO	-	-	I/O	CHI
    /// 22	10110	RAM	-	CLO	-	-	I/O	CHI
    /// 23	10111	RAM	-	CLO	-	-	I/O	CHI
    /// 24	11000	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 25	11001	RAM	RAM	RAM	RAM	RAM	CHR	RAM
    /// 26	11010	RAM	RAM	RAM	RAM	RAM	CHR	KRN
    /// 27	11011	RAM	RAM	RAM	BSC	RAM	CHR	KRN
    /// 28	11100	RAM	RAM	RAM	RAM	RAM	RAM	RAM
    /// 29	11101	RAM	RAM	RAM	RAM	RAM	I/O	RAM
    /// 30	11110	RAM	RAM	RAM	RAM	RAM	I/O	KRN
    /// 31	11111	RAM	RAM	RAM	BSC	RAM	I/O	KRN
    ///
    /// B1: $0000 - $0fff
    /// B2: $1000 - $7fff
    /// B3: $8000 - $9fff
    /// B4: $a000 - $bfff
    /// B5: $c000 - $cfff
    /// B5: $d000 - $dfff
    /// B7: $e000 - $ffff
    ///
    /// RAM: RAM
    /// CLO: Cartridge ROM (lo)
    /// CHI: Character ROM
    /// BSC: Basic ROM
    /// I/O: CIA1 or CIA2
    /// CHI: Cartridge ROM (hi)
    /// KRN: Kernal ROM
    /// ```
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

    // start address per memory device:   RAM  CART LO  BASIC  CART HI     IO CHARGEN KERNAL
    static ref START_ADDR: [u16; 7] = [0x0000, 0x8000, 0xa000, 0xa000, 0xd000, 0xd000, 0xe000];
}

const RAM: usize = 0;
const CARTRIDGE_LO: usize = 1;
const BASIC: usize = 2;
const CARTRIDGE_HI: usize = 3;
const IO: usize = 4;
const CHARGEN: usize = 5;
const KERNAL: usize = 6;
const INVALID: usize = 7;

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

// TODO
// Corrections are still required  for addresses 0x0000 and 0x0001
// as they are not in RAM, but are internal CPU states
// see here:
//
// CONSIDERATIONS
// Proper behavior should be defined for som devices being missing
// The options are
// 1. Fallback to X (most likely RAM, if present)
// 2. Retun 0 or "rubbish"
// 3. Fail
// Although 3 feels natural, it's not neccessary the reality.
// There is informative topic on lemon64.com about testing real C64 with some
// chips removed:
// https://www.lemon64.com/forum/viewtopic.php?t=78824
// Here is my compilation of observed results
// No U1 (CIA1) - Screen fine, no cursor (as no IRQs)
// No U2 (CIA2) - OK
// No U1/U2 - Random characters on the screen
// No U4 (KERNAL) - Blank screen
// No U5 (CharROM) - blue screen with borders
// No U18(SID) - OK
//
// We know (source?) that write operation always fallsback to RAM
// (whether addressed device present or not - the only exception is writing
// to I/O, which - obviously - is allowed)
//
// But how about read? The choice between option 1 or 2 needs to be made.
// Perhaps it's possible to figure it out from PLA schematics
// From emulation perspective both options have values
// Fallback to RAM would let to work without other devices implemented
// that may be handy at the early stage of emu development
// Fallback to 0 woudld let to spot problems with missing devices.
// I somehow sense (can't prove) the latter is closer to reality.

type Cell = Arc<Mutex<dyn Addressable + Send>>;
type OptCell = Option<Cell>;

#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct PLA_82S100 {
    devices: [OptCell; 7],
}

impl Addressable for PLA_82S100 {
    fn read_byte(&self, addr: Addr) -> u8 {
        let (byte0, byte1, ram) = self.get_state();
        let id = self.get_device_id(addr, byte0, byte1);

        if id == INVALID {
            // TODO check what to do in such this case
            return 0;
        }

        let real_id = if_else(self.has_device(id), id, 0);

        // optimization - if read is from RAM no further checks are required
        // so we can skip further mutex locking
        if real_id == RAM || real_id == IO {
            return ram.read_byte(addr);
        }

        let opt_dev = &self.devices[real_id as usize];
        if let Some(dev) = opt_dev {
            let real_addr = self.internal_addr(&dev, addr, real_id);
            dev.lock().unwrap().read_byte(real_addr)
        } else {
            0
        }
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        let (byte0, byte1, mut ram) = self.get_state();
        let id = self.get_device_id(addr, byte0, byte1);
        if id == INVALID || !self.can_write(addr, byte0) {
            // TODO check what to do in case of INVALID
            return ();
        }

        let real_id = if_else(id == IO, IO, RAM); // if not i/o, write to ram

        // optimization - if write is to RAM, no further checks are required
        // so we can avoid additional mutex locking
        if real_id == RAM {
            return ram.write_byte(addr, value);
        }

        drop(ram);
        if self.has_device(real_id) {
            let internal_addr = {
                let dev = self.devices[real_id as usize].as_ref().unwrap();
                self.internal_addr(&dev, addr, real_id)
            };
            let dev_mut = self.devices[real_id as usize].as_mut().unwrap();
            dev_mut.lock().unwrap().write_byte(internal_addr, value);
        }
    }

    fn address_width(&self) -> u16 {
        16
    }
}

impl AddressResolver for PLA_82S100 {}

impl PLA_82S100 {
    fn get_state(&self) -> (u8, u8, MutexGuard<dyn Addressable + Send + 'static>) {
        if !self.has_device(RAM) {
            panic!("RAM is mandatory");
        }
        let ram = lock(self.devices[RAM].as_ref().unwrap());
        let byte0 = ram.read_byte(0);
        let byte1 = ram.read_byte(1);
        (byte0, byte1, ram)
    }

    fn get_device_id(&self, addr: Addr, _byte0: u8, byte1: u8) -> usize {
        // pin 8 and 9 are set low (false) when cartridge is present and high (true) when not
        // regular cartridge: pin 8
        // exrom: pin 8 and 9
        let pin8 = !self.has_device(CARTRIDGE_LO);
        let pin9 = !self.has_device(CARTRIDGE_HI);

        // flag is a combination of 3 youngest bits from processor port 0x01
        // and values from pin8 and 9, that act here as bit 4 and 5
        // that gives 32 combinations (although some of them are redundant, so
        // effectively there is 14)
        let flag = (byte1 & 0b111) | (u8::from(pin8) << 3) | (u8::from(pin9) << 4);
        let bank = &BANKS[flag as usize];
        let dev_id = match addr {
            0x0000..=0x0fff => bank[0],
            0x1000..=0x7fff => bank[1],
            0x8000..=0x9fff => bank[2],
            0xa000..=0xbfff => bank[3],
            0xc000..=0xcfff => bank[4],
            0xd000..=0xdfff => bank[5],
            0xe000..=0xffff => bank[6],
        } as usize;

        dev_id
    }

    fn has_device(&self, dev_id: usize) -> bool {
        self.devices[dev_id].is_some()
    }

    /// Converts provided address into "local address" (think of array cell id)
    /// of a specific addressable device.
    /// I.e. BASIC ROM has only 8kB, but it operates in the address space starting
    /// at $A000. Means we need a mechanism that maps such addresses into 8kB
    /// memory cells of BASIC ROM. In practice it's just subtraction.
    fn internal_addr(&self, _dev: &Cell, addr: Addr, id: usize) -> Addr {
        // if id == IO && (addr >= 0xdc00 || addr <= 0xdcff) {
        //     return addr - 0xdc00;
        // }
        // a & (dev.address_width() - 0)
        addr - START_ADDR[id]
    }

    // byte0 (Address $0000) determines whether some processor lines
    // are available as input, means read-only (0) or output, means R/W (1).
    // In practice each bit of $0000 coresponds to bits on $0001
    // From the context of PLA82S100 it means
    // bit 0: controls LORAM, so BASIC area ($A000-$BFFF)
    // bit 1: controls HIRAM, so KERNAL area ($E000-$FFFF)
    // bit 2: controls IO area ($D000-$DFFF)
    // Of course for the purpose of write we can skip BASIC
    // or KERNAL ROMs, but if RAM or IO is available under these
    // areas, the the bits from byte0 control the write access.
    // At leasr if I got it right based on
    // 1. https://www.c64-wiki.com/wiki/Zeropage
    // 2. https://www.pagetable.com/c64ref/c64mem/
    fn can_write(&self, addr: Addr, byte0: u8) -> bool {
        if ((byte0 & 0b001) == 0 && (0xa000..=0xbfff).contains(&addr))
            || ((byte0 & 0b010) == 0 && (0xe000..=0xffff).contains(&addr))
            || ((byte0 & 0b100) == 0 && (0xd000..=0xdfff).contains(&addr))
        {
            false
        } else {
            true
        }
    }

    #[cfg(test)]
    pub(crate) fn set_mode(&mut self, mode: u8) {
        self.write_byte(1, mode);
    }

    pub(crate) fn link_dev(&mut self, id: usize, dev: Cell) -> &mut Self {
        self.devices[id] = Some(dev.clone());
        self
    }

    pub fn link_ram(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(RAM, dev)
    }
    pub fn link_basic(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(BASIC, dev)
    }
    pub fn link_kernal(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(KERNAL, dev)
    }
    pub fn link_chargen(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(CHARGEN, dev)
    }
    pub fn link_cartridge(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(CARTRIDGE_LO, dev)
    }
    pub fn link_io(&mut self, dev: Cell) -> &mut Self {
        self.link_dev(IO, dev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulator::abstractions::Addressable;

    struct Mem {
        width: u16,
        pub data: [u8; 0xffff],
    }

    impl Addressable for Mem {
        fn read_byte(&self, addr: Addr) -> u8 {
            self.data[addr as usize]
        }

        fn write_byte(&mut self, addr: Addr, value: u8) {
            self.data[addr as usize] = value;
        }

        fn address_width(&self) -> u16 {
            self.width
        }
    }

    impl Mem {
        fn new(width: u16) -> Self {
            Mem {
                width,
                data: [0; 0xffff],
            }
        }
    }

    #[test]
    fn test_creation() {
        let ram = Mem::new(16);
        let mut pla = PLA_82S100::default();
        pla.link_ram(Arc::new(Mutex::new(ram)));
        assert_eq!(0, pla.read_byte(0x33));
        pla.write_byte(0x33, 42);
        assert_eq!(42, pla.read_byte(0x33));
    }

    #[test]
    fn test_operations() {
        let ram = Mem::new(16);
        let basic = Mem::new(16);
        let chargen = Mem::new(16);
        let kernal = Mem::new(16);

        let mut pla = PLA_82S100::default();
        pla.link_dev(RAM, Arc::new(Mutex::new(ram)));
        pla.link_dev(BASIC, Arc::new(Mutex::new(basic))); // basic rom
        pla.link_dev(CHARGEN, Arc::new(Mutex::new(chargen))); // char rom
        pla.link_dev(KERNAL, Arc::new(Mutex::new(kernal))); // kernal

        // read/write ram
        assert_eq!(0, pla.read_byte(0x33));
        pla.write_byte(0x33, 42);
        assert_eq!(42, pla.read_byte(0x33));

        // read/write basic rom
        pla.set_mode(0);
        pla.write_byte(0xa000, 42); // write to basic scope
                                    // try to read from basic scope, but in mode 0 there is ram
        assert_eq!(42, pla.read_byte(0xa000));

        pla.set_mode(0b11111); // switch to mode 31
        assert_eq!(0, pla.read_byte(0xa000)); // now we read from rom
        pla.write_byte(0xa000, 66); // but we can still write to ram
        assert_eq!(0, pla.read_byte(0xa000));
        pla.set_mode(0);
        assert_eq!(66, pla.read_byte(0xa000));
    }
}
