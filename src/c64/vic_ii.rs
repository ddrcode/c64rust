#![allow(non_camel_case_types)]

use super::*;
use colored::*;

// see https://c64os.com/post/c64screencodes
const SCREEN_CODES: &str = "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
                            -·······························································\
                            @abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
                            -ABCDEFGHIJKLMNOPQRSTUVWXYZ·····································";

pub struct VIC_II {}

impl VIC_II {
    pub fn print_screen(&self, mem: &Memory) {
        let chars: Vec<char> = SCREEN_CODES.chars().collect();
        let mut n = 0;
        println!();
        println!("{}", " ".repeat(44).on_truecolor(0x6c, 0x5e, 0xb5));
        print!("{}", "  ".on_truecolor(0x6c, 0x5e, 0xb5));
        for i in 0x0400..0x07e8 {
            let sc = mem.get_byte(i) as usize;
            print!("{}", format!("{}", chars[sc]).on_truecolor(0x35, 0x28, 0x79));
            n += 1;
            if n % 40 == 0 {
                print!("{}", "  ".on_truecolor(0x6c, 0x5e, 0xb5));
                println!();
                print!("{}", "  ".on_truecolor(0x6c, 0x5e, 0xb5));
            }
        }
        println!("{}", " ".repeat(42).on_truecolor(0x6c, 0x5e, 0xb5));
        println!("              ");
    }
}

