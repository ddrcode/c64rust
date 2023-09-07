#![allow(non_camel_case_types)]

use super::*;
use colored::*;

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
            print!(
                "{}",
                format!("{}", chars[sc]).on_truecolor(0x35, 0x28, 0x79)
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
