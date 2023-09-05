use std::fs::{ File, metadata };
use std::io::Read;
use std::env;

mod mos6510;
mod c64;
mod machine;

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
    println!("C64 EMU. {} operations implemented", c64.cpu.operations.len());

    // c64.load(&[0x69, 0x05, 0x69, 0x07, 0x00], 0x0100);
    // c64.run(0x0100);
    c64.run(0xe000); // start KERNAL

    println!("Accumulator: {}", c64.cpu.registers.accumulator);
}
