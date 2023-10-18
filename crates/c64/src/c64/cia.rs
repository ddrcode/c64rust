use super::keyboard::Keyboard;
use machine::emulator::{
    abstractions::{Addr, Addressable, DeviceTrait},
    components::{CIA_6526, TOD},
};

// -----------------------------------------
// CIA1

pub struct CIA1 {
    data: [u8; 16],
    t_a: u16,
    t_b: u16,
    time: TOD,
    pub keyboard: Keyboard,
}

impl CIA1 {
    pub fn new() -> CIA1 {
        let mut data = [0u8; 16];
        data[2] = 0xff;
        CIA1 {
            data,
            t_a: 0,
            t_b: 0,
            time: TOD::new(),
            keyboard: Keyboard::new(),
        }
    }
}

impl CIA_6526 for CIA1 {
    fn mem(&self) -> &[u8] {
        &self.data
    }

    fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        if addr == 0x00 {
            self.data[0] = val;
            let code = self.keyboard.scan(val, self.data[1]); // self.read_byte(0xdc01));
            self.data[1] = code;
            return ();
        }
        self.mem_mut()[addr as usize] = val;
    }

    fn timer_a(&self) -> u16 {
        self.t_a
    }

    fn timer_b(&self) -> u16 {
        self.t_b
    }

    fn tick(&mut self) {
        self.t_a = self.t_a.wrapping_sub(1);
        self.t_b = self.t_b.wrapping_sub(1);
    }

    fn tod(&self) -> &TOD {
        &self.time
    }
}

// such a nonsense!!
impl Addressable for CIA1 {
    fn read_byte(&self, addr: Addr) -> u8 {
        CIA_6526::read_byte(self, addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        CIA_6526::write_byte(self, addr, value);
    }

    fn address_width(&self) -> u16 {
        CIA_6526::address_width(self)
    }
}

impl DeviceTrait for CIA1 {}

// -----------------------------------------
// CIA2

pub struct CIA2 {
    data: [u8; 16],
    t_a: u16,
    t_b: u16,
    time: TOD
}

impl CIA2 {
    pub fn new() -> CIA2 {
        let mut data = [0u8; 16];
        data[2] = 0xff;
        CIA2 {
            data,
            t_a: 0,
            t_b: 0,
            time: TOD::new()
        }
    }
}

impl CIA_6526 for CIA2 {
    fn mem(&self) -> &[u8] {
        &self.data
    }

    fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn timer_a(&self) -> u16 {
        self.t_a
    }

    fn timer_b(&self) -> u16 {
        self.t_b
    }

    fn tick(&mut self) {
        self.t_a = self.t_a.wrapping_sub(1);
        self.t_b = self.t_b.wrapping_sub(1);
    }

    fn tod(&self) -> &TOD {
        &self.time
    }
}

// such a nonsense!!
impl Addressable for CIA2 {
    fn read_byte(&self, addr: Addr) -> u8 {
        CIA_6526::read_byte(self, addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        CIA_6526::write_byte(self, addr, value);
    }

    fn address_width(&self) -> u16 {
        CIA_6526::address_width(self)
    }
}

impl DeviceTrait for CIA2 {}
