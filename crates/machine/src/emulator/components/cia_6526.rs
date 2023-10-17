use crate::{
    emulator::abstractions::{Addr, Addressable},
    utils::if_else,
};
use bcd_numbers::BCD;
use chrono::Timelike;

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
#[allow(non_camel_case_types)]
pub trait CIA_6526: Addressable {
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
        }
        else if (0x04..=0x07).contains(&addr) {
            match addr {
                0x04 => self.timer_a().to_le_bytes()[0],
                0x05 => self.timer_a().to_le_bytes()[1],
                0x06 => self.timer_b().to_le_bytes()[0],
                0x07 => self.timer_b().to_le_bytes()[1],
                _ => panic!("Shouldn't happen")
            }
        }
        else {
            self.mem()[addr as usize]
        }
    }

    fn write_byte(&mut self, addr: Addr, val: u8) {
        if (0x04..=0x07).contains(&addr) {
            log::info!("Setting timer at {:04x} for {val}", addr);
        }
        self.mem_mut()[addr as usize] = val;
    }

    fn address_width(&self) -> u16 {
        4
    }

    fn tick(&mut self);

    fn tick_times(&mut self, times: u8) {
        for _ in 0..times {
            self.tick();
        }
    }

    fn timer_a(&self) -> u16;
    fn timer_b(&self) -> u16;
}
