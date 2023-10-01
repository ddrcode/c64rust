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

use crate::{
    emulator::abstractions::{Addr, AddressResolver, Addressable},
    utils::if_else,
};

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
// No U4 (???) - Blank screen
// No U5 (CharROM) - blue screen with borders
// No U18(???) - OK
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

#[derive(Default)]
pub struct PLA_82S100<'a> {
    devices: [Option<Box<&'a dyn Addressable>>; 7],
}

impl<'a> Addressable for PLA_82S100<'a> {
    fn read_byte(&self, addr: Addr) -> u8 {
        let id = self.get_device_id(addr);
        if id == 7 {
            return 0;
        } // TODO check what to do for this case
        let opt_dev = self.devices[id as usize].as_ref().or(self.devices[0].as_ref()); // fallback to RAM
        if let Some(dev) = opt_dev {
            dev.read_byte(self.internal_addr(dev, addr))
        } else {
            0
        }
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        // TODO I think byte 0 may prevent writing at all - TBC
        // let byte0 = self.device(0).read_byte(0x0000);

        let id = self.get_device_id(addr);
        let real_id = if_else(id == 4, 4, 0); // if not i/o, write to ram
        if let Some(dev) = &self.devices[real_id] {
            // dev.write_byte(self.internal_addr(&mut dev, addr), value);
        }
    }

    fn address_width(&self) -> u16 {
        16
    }
}

impl<'a> AddressResolver for PLA_82S100<'a> {}

impl<'a> PLA_82S100<'a> {
    fn get_device_id(&self, addr: Addr) -> u8 {
        // pin 8 and 9 are activated when cartridge is present
        // regular cartrisge: pin 8
        // exrom: pin 8 and 9
        let pin8 = self.has_device(1) || self.has_device(3);
        let pin9 = self.has_device(3);

        // Because we are emulating addresses 0 and 1 with RAM
        // we can't continue when RAM is not present.
        if self.devices[0].is_none() {
            return 0;
        }

        // flag is a combination of 3 youngest bits from processor port 0x01
        // and values from pin8 and 9, that act here as bit 4 and 5
        // that gives 32 combinations (although some of them are redundant, so
        // effectively there is 14)
        let mut flag = self.devices[0].as_ref().unwrap().read_byte(0x0001);

        flag = (flag & 0b111) | (u8::from(pin8) << 4) | (u8::from(pin9) << 5);
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

    fn has_device(&self, dev_id: usize) -> bool {
        self.devices[dev_id].is_some()
    }

    fn internal_addr(&self, dev: &Box<&dyn Addressable>, addr: Addr) -> Addr {
        addr & (dev.address_width() - 1)
    }
}

#[cfg(test)]
mod tests{
    use std::sync::{Mutex, Arc};

    use crate::emulator::abstractions::{RAM, Addressable};

    use super::*;

    struct Ram {
    }
    impl Addressable for Ram{
        fn read_byte(&self, addr: Addr) -> u8 {
            42
        }

        fn write_byte(&mut self, addr: Addr, value: u8) {
            todo!()
        }

        fn address_width(&self) -> u16 {
            16
        }
    }

    #[test]
    fn test_creation() {
        let ram = Ram{};
        let pla = PLA_82S100 {
            devices: [Some(Box::new(&ram)), None, None, None, None, None, None]
        };
        assert_eq!(42, pla.read_byte(0x33));
    }


    #[test]
    fn test_mutex() {
        let ram = Ram{};
        let pla = PLA_82S100 {
            devices: [Some(Box::new(&ram)), None, None, None, None, None, None]
        };
        let xtr = Arc::new(Mutex::new(pla));
        assert_eq!(42, xtr.lock().unwrap().read_byte(0x200));
    }
}
