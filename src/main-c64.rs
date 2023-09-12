extern crate colored;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod c64;
mod cli_args;
mod machine;
mod mos6510;

#[cfg(test)]
mod tests;

use crate::c64::C64;
use crate::cli_args::Args;
use crate::machine::{Machine, MachineConfig};

fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() {
    let args = Args::parse();
    let mut c64 = C64::new(MachineConfig::from(&args));

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        c64.init_rom(&rom[..]);
    }

    c64.power_on();

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        c64.machine.mem.write(addr, &ram[..]);
    }

    c64.start();


    if args.show_status {
        println!("{}", c64.machine.cpu.registers);
    }

    if args.show_screen {
        c64.print_screen();
    }
}
