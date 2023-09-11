extern crate colored;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod cli_args;
mod machine;
mod mos6510;

#[cfg(test)]
mod tests;

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
    let mut machine = Machine::new(MachineConfig::from(&args));

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        machine.mem.init_rom(&rom[..]);
    }

    machine.power_on();

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        machine.mem.write(addr, &ram[..]);
    }

    // machine.run(u16::from_str_radix(&args.start_addr, 16).unwrap()); // start KERNAL
    machine.start();

    if args.show_status {
        println!("{}", machine.cpu.registers);
    }

    // if args.show_screen {
    //     machine.print_screen();
    // }
}
