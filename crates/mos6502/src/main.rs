extern crate colored;

mod cli;
mod machine;
mod mos6502;

use crate::cli::{get_file_as_byte_vec, Args};
use crate::machine::{MOS6502Machine, Machine, MachineConfig};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let mut machine = MOS6502Machine::new(MachineConfig::from(&args));

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        machine.memory_mut().init_rom(&rom[..]);
    }

    machine.power_on();

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        machine.memory_mut().write(addr, &ram[..]);
    }

    machine.start();

    if args.show_status {
        println!("{}", machine.cpu().registers);
    }
}
