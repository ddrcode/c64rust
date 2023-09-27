extern crate colored;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod c64;
mod client;
mod key_utils;

use crate::c64::C64;
use crate::client::C64Client;
use machine::client::{ClientError, NonInteractiveClient};
use machine::{cli::Args, Machine, MachineConfig};

fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() -> Result<(), ClientError> {
    let args = Args::parse();
    let mut c64 = C64::new(MachineConfig::from(&args));

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        c64.memory_mut().init_rom(&rom[..]);
    }

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        c64.memory_mut().write(addr, &ram[..]);
    }

    let mut client = C64Client::new(c64);
    client.start_sync()?;

    if args.show_status {
        println!("{}", client.get_cpu_state().unwrap());
    }

    if args.show_screen {
        client.mutex().lock().unwrap().print_screen();
    }

    client.stop()
}
