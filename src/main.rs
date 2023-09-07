extern crate colored;

use std::env;
use std::fs::File;
use std::io::Read;

mod c64;
mod mos6510;

#[cfg(test)]
mod tests;

use crate::c64::C64;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = get_file_as_byte_vec(&args[1]);

    let mut c64 = C64::new();
    c64.mem.init_rom(&rom[..]);
    c64.power_on();

    c64.run(0xfce2); // start KERNAL
    c64.print_screen();
}
