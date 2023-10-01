use core::num::Wrapping;

pub type Addr = u16;

pub trait Addressable {
    fn writeable() -> bool
    where
        Self: Sized;

    fn read_byte(&self, addr: Addr) -> u8;
    fn write_byte(&mut self, addr: Addr, value: u8);

    fn read_byte_wrapping(&self, addr: Addr) -> Wrapping<u8> {
        Wrapping(self.read_byte(addr))
    }
}

pub trait RAM: Addressable {
    fn writeable() -> bool
    where
        Self: Sized,
    {
        true
    }
}

pub trait ROM: Addressable {
    fn init(data: &[u8]) -> Self;

    fn writeable() -> bool {
        false
    }

    fn write_byte(&mut self, addr: Addr) {
        panic!("Attempt to write write-only addressable device (at addr {addr})");
    }
}

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
