#![allow(non_camel_case_types)]

use crate::key_utils::screen_code_to_ascii;
use colored::*;
use machine::Memory;
use super::C64Memory;

/*
 * RGB values of C64 colours
 * #000000 black
 * #FFFFFF white
 * #68372B red
 * #70A4B2 light blue
 * #6F3D86 purple
 * #588D43 green
 * #352879 dark blue
 * #B8C76F yellow
 * #6F4F25 brown
 * #433900 dark brown
 * #9A6759 light red
 * #444444 dark grey
 * #6C6C6C mid grey
 * #9AD284 light green
 * #6C5EB5 mid blue
 * #959595 light grey
 */
pub struct VIC_II {
    char_set: u8,
}

impl VIC_II {
    pub fn new() -> Self {
        VIC_II { char_set: 0x14 }
    }

    pub fn print_screen(&self, mem: &C64Memory) {
        let mut n = 0;
        println!();
        println!("{}", " ".repeat(44).on_truecolor(0x6c, 0x5e, 0xb5));
        print!("{}", "  ".on_truecolor(0x6c, 0x5e, 0xb5));
        for i in 0x0400..0x07e8 {
            let sc = mem.get_byte(i);
            print!(
                "{}",
                format!("{}", screen_code_to_ascii(&sc, self.char_set))
                    .on_truecolor(0x35, 0x28, 0x79)
            );
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
