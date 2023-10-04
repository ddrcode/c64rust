use std::time::SystemTime;

use super::Keyboard;
use bcd_numbers::BCD;
use chrono::Timelike;
use machine::{
    emulator::abstractions::{Addressable, DeviceTrait},
    utils::if_else,
    Addr,
};

/// To find out more about CIA6526, read here
/// [CIA #1 in Mapping C64](http://www.unusedino.de/ec64/technical/project64/mapping_c64.html)
/// [CIA at C64Wiki](https://www.c64-wiki.com/wiki/CIA)
/// [CIAs - Timers, Keyboard and more](https://emudev.de/q00-c64/cias-timers-keyboard-and-more/)
/// [Data sheet](http://archive.6502.org/datasheets/mos_6526_cia_recreated.pdf)
///
/// Current implementation status
/// $00 (Port A) - Done for CIA 1 (Keyboard)
/// $01 (Port B) - Done for CIA 1 (Keyboard)
/// $02 (Port A flags) - Hardcoded to $FF
/// $03 (Port B flags) - Hardcoded to 0
/// $04-$07 (Timer and and B) - TO DO
/// $08-$0B (RTC) - Read only
/// $0C (Serial shift register) - TO DO
/// $0D (Interrupt control and status) - TO DO
/// $0E-$0F (Timer control) - TO DO
pub trait CIA6526: Addressable {
    fn mem(&self) -> &[u8];
    fn mem_mut(&mut self) -> &mut [u8];

    fn read_byte(&self, addr: Addr) -> u8 {
        if (0x8..=0xb).contains(&addr) {
            let now = chrono::Local::now().time();
            match addr {
                0x08 => {
                    BCD::<1>::new(((now.nanosecond() / 100_000_000) % 10).into()).get_number() as u8
                }
                0x09 => BCD::<1>::new(now.second().into()).get_number() as u8,
                0x0a => BCD::<1>::new(now.minute().into()).get_number() as u8,
                0x0b => {
                    let (pm, hour) = now.hour12();
                    let pm_flag = if_else(pm, 128, 0);
                    let bcd = (BCD::<2>::new(hour.into()).get_number()) as u8;
                    bcd | pm_flag
                }
                _ => 0,
            }
        } else {
            self.mem()[addr as usize]
        }
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        self.mem_mut()[addr as usize] = val;
    }

    fn address_width(&self) -> u16 {
        4
    }
}

// -----------------------------------------
// CIA1

pub struct CIA1 {
    data: [u8; 16],
    pub keyboard: Keyboard,
}

impl CIA1 {
    pub fn new() -> CIA1 {
        let mut data = [0u8; 16];
        data[2] = 0xff;
        CIA1 {
            data,
            keyboard: Keyboard::new(),
        }
    }
}

impl CIA6526 for CIA1 {
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
}

// such a nonsense!!
impl Addressable for CIA1 {
    fn read_byte(&self, addr: Addr) -> u8 {
        CIA6526::read_byte(self, addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        CIA6526::write_byte(self, addr, value);
    }

    fn address_width(&self) -> u16 {
        CIA6526::address_width(self)
    }
}

impl DeviceTrait for CIA1 {}

// -----------------------------------------
// CIA2

pub struct CIA2 {
    data: [u8; 16],
}

impl CIA2 {
    pub fn new() -> CIA2 {
        let mut data = [0u8; 16];
        data[2] = 0xff;
        CIA2 { data }
    }
}

impl CIA6526 for CIA2 {
    fn mem(&self) -> &[u8] {
        &self.data
    }

    fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}


// such a nonsense!!
impl Addressable for CIA2 {
    fn read_byte(&self, addr: Addr) -> u8 {
        CIA6526::read_byte(self, addr)
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        CIA6526::write_byte(self, addr, value);
    }

    fn address_width(&self) -> u16 {
        CIA6526::address_width(self)
    }
}

impl DeviceTrait for CIA2 {}
