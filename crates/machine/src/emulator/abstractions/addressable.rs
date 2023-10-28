use super::async_device::*;
use core::num::Wrapping;

pub type Addr = u16;

pub trait Addressable {
    fn read_byte(&self, addr: Addr) -> u8;
    fn write_byte(&mut self, addr: Addr, value: u8);
    fn address_width(&self) -> u16;

    fn read_byte_wrapping(&self, addr: Addr) -> Wrapping<u8> {
        Wrapping(self.read_byte(addr))
    }
}

/// BankSwitch should NEVER expose any of Addressables, it switches between
pub trait AddressResolver: Addressable {
    fn fragment(&self, from: Addr, to: Addr) -> Vec<u8> {
        let mut vec = Vec::<u8>::with_capacity((to - from) as usize);
        let range = std::ops::Range {
            start: from,
            end: to,
        };
        for i in range {
            // this is quite suboptimal as mutex locking
            // is involved in every read
            vec.push(self.read_byte(i));
        }
        vec
    }

    fn read_word(&self, addr: Addr) -> u16 {
        u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr.wrapping_add(1))])
    }
}

impl<T: DeviceTrait + Addressable> Addressable for Device<T> {
    fn read_byte(&self, addr: Addr) -> u8 {
        self.lock().read_byte(addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        self.lock().write_byte(addr, value)
    }

    fn address_width(&self) -> u16 {
        self.lock().address_width()
    }
}

pub trait AddressableDevice<T: DeviceTrait>: Addressable + Accessor<T> {}
impl<T: Addressable + DeviceTrait> AddressableDevice<T> for Device<T> {}

/// Simple implementation of Array built on top of u8 array.
pub struct ArrayMemory {
    cells: Box<[u8]>,
    width: u16,
}

impl ArrayMemory {
    pub fn new(size: usize, width: u16) -> Self {
        ArrayMemory {
            cells: vec![0u8; size].into_boxed_slice(),
            width,
        }
    }

    pub fn from_data(data: &[u8], width: u16) -> Self {
        ArrayMemory {
            cells: Vec::from(data).into_boxed_slice(),
            width,
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

impl Addressable for ArrayMemory {
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

impl DeviceTrait for ArrayMemory {}
