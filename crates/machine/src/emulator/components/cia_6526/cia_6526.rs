use super::TOD;
use crate::emulator::abstractions::{Addr, Addressable, Tickable};

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
/// $04-$07 (Timer A and B) - TO DO
/// $08-$0B (TOD: Time-of-day) - Read only
/// $0C (Serial shift register) - TO DO
/// $0D (Interrupt control and status) - TO DO
/// $0E-$0F (Timer control) - TO DO
#[allow(non_camel_case_types)]
pub trait CIA_6526: Addressable + Tickable {
    fn mem(&self) -> &[u8];
    fn mem_mut(&mut self) -> &mut [u8];

    fn read_byte(&self, addr: Addr) -> u8 {
        if (0x8..=0xb).contains(&addr) {
            match addr {
                0x08 => self.tod().tenth(),
                0x09 => self.tod().second(),
                0x0a => self.tod().minute(),
                0x0b => self.tod().hour(),
                _ => 0,
            }
        } else if (0x04..=0x07).contains(&addr) {
            match addr {
                0x04 => self.timer_a().to_le_bytes()[0],
                0x05 => self.timer_a().to_le_bytes()[1],
                0x06 => self.timer_b().to_le_bytes()[0],
                0x07 => self.timer_b().to_le_bytes()[1],
                _ => panic!("Shouldn't happen"),
            }
        } else {
            self.mem()[addr as usize]
        }
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        if (0x04..=0x07).contains(&addr) {
            log::info!("Setting timer at {:04x} for {val}", addr);
        }

        if (0x8..=0xb).contains(&addr) {
            match addr {
                0x08 => self.tod_mut().set_tenth(val),
                0x09 => self.tod_mut().set_second(val),
                0x0a => self.tod_mut().set_minute(val),
                0x0b => self.tod_mut().set_hour(val),
                _ => (),
            };
        } else {
            self.mem_mut()[addr as usize] = val;
        }
    }

    fn address_width(&self) -> u16 {
        4
    }

    fn timer_a(&self) -> u16;
    fn timer_b(&self) -> u16;
    fn tod(&self) -> &TOD;
    fn tod_mut(&mut self) -> &mut TOD;
}

