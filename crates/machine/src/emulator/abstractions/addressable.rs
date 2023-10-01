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

pub trait RAM: Addressable {
    fn address_width(&self) -> u16 { 16 }
}

pub trait ROM: Addressable {
    fn init(data: &[u8]) -> Self;

    fn write_byte(&mut self, _addr: Addr) {
        // any attempt to write will be simply ignored
        // no error is needed
    }

    fn address_width(&self) -> u16 { 16 }
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
