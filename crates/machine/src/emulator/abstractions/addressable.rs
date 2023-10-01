use core::num::Wrapping;
use std::ops::Index;

pub type Addr = u16;

pub trait Addressable {

    fn read_byte(&self, addr: Addr) -> u8;
    fn write_byte(&mut self, addr: Addr, value: u8);
    fn address_width(&self) -> u16;

    fn read_byte_wrapping(&self, addr: Addr) -> Wrapping<u8> {
        Wrapping(self.read_byte(addr))
    }
}

pub trait IndexedAddressible<T>: Addressable+Index<T, Output=u8> {
}

// impl<T: IndexedAddressible<T>> Addressable for T {
//     fn address_width(&self) -> u16 { 16 }
//
//     fn read_byte(&self, addr: Addr) -> u8 {
//         *self.index(addr as usize)
//     }
//
//     fn write_byte(&mut self, addr: Addr, value: u8) {
//         todo!()
//     }
// }


/// BankSwitch should NEVER expose any of Addressables it switches betweenop
pub trait AddressResolver: Addressable {
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

    fn read_word(&self, addr: Addr) -> u16 {
        u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr.wrapping_add(1))])
    }
}


pub struct VecMemory {
    cells: Vec<u8>,
    width: u16
}

impl VecMemory {
    pub fn new(size: usize, width: u16) -> Self {
        // VecMemory {
        //     cells:Vec::with_capacity(size),
        //     width
        // }
        VecMemory::from_data(&[0u8;1<<16], width)
    }

    pub fn from_data(data: &[u8], width: u16) -> Self {
        VecMemory {
            cells: Vec::from(data),
            width
        }
    }
}

impl Addressable for VecMemory {
    fn read_byte(&self, addr: Addr) -> u8 {
        self.cells[addr as usize]
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        self.cells[addr as usize] = value;
    }

    fn address_width(&self) -> u16 {
        self.width
    }
}
