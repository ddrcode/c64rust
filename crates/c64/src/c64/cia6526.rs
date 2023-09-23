use super::Keyboard;
use machine::Addr;

pub struct CIA1 {
    address: Addr,
    data: [u8; 16],
    pub keyboard: Keyboard,
}

impl CIA1 {
    pub fn new(addr: Addr) -> CIA1 {
        CIA1 {
            address: addr,
            data: [0; 16],
            keyboard: Keyboard::new(),
        }
    }
}

pub trait CIA6526 {
    fn get_base_addr(&self) -> u16;
    fn mem(&self) -> &[u8];
    fn mem_mut(&mut self) -> &mut [u8];

    fn get_addr(&self, addr: Addr) -> usize {
        let address = self.get_base_addr();
        if addr < 16 {
            return addr as usize;
        }
        if addr < address || addr > address + 15 {
            panic!(
                "Requested CIA address ({:04x}) is outside of range ({:04x} - {:04x})",
                addr,
                address,
                address + 15
            );
        }
        (addr - address) as usize
    }

    fn get_byte(&self, addr: Addr) -> u8 {
        self.mem()[self.get_addr(addr)]
    }

    fn set_byte(&mut self, addr: Addr, val: u8) {
        let address = self.get_addr(addr);
        self.mem_mut()[address] = val;
    }
}

impl CIA6526 for CIA1 {
    fn get_base_addr(&self) -> u16 {
        self.address
    }

    fn mem(&self) -> &[u8] {
        &self.data
    }

    fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn set_byte(&mut self, addr: Addr, val: u8) {
        let address = self.get_addr(addr);
        if addr == 0xdc00 {
            let code = self.keyboard.scan(val, self.get_byte(0xdc01));
            self.mem_mut()[address + 1] = code;
        }
        self.mem_mut()[address] = val;
    }
}
