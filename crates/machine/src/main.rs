extern crate colored;

mod cli;
mod client;
mod machine;
mod mos6502;
mod utils;

use crate::cli::{get_file_as_byte_vec, Args};
use crate::client::{ClientError, DirectClient, NonInteractiveClient};
use crate::machine::{MOS6502Machine, Machine, MachineConfig};
use clap::Parser;

fn main() -> Result<(), ClientError> {
    let args = Args::parse();
    let mut machine = MOS6502Machine::new(MachineConfig::from(&args));

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        machine.memory_mut().init_rom(&rom[..]);
    }

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        machine.memory_mut().write(addr, &ram[..]);
    }

    let mut client = DirectClient::new(machine);
    client.start_sync()?;

    if args.show_status {
        println!("{}", client.get_cpu_state().unwrap());
    }

    client.stop()
}
