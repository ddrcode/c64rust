use crate::machine::{FromConfig, Machine, MachineConfig, Memory};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use super::Args;
use clap::Parser;
use anyhow::Result;

pub fn get_file_as_byte_vec(filename: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn create_machine_from_cli_args<M>() -> Result<M>
where
    M: FromConfig + Machine,
{
    let args = Args::parse();
    let config = MachineConfig::from(&args);

    let mut machine = M::from_config(config);

    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file)?;
        machine.memory_mut().init_rom(&rom[..]);
    }

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file)?;
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        machine.memory_mut().write(addr, &ram[..]);
    }

    if let Some(character_rom) = args.character_rom {
        let rom = get_file_as_byte_vec(&character_rom)?;
        machine.memory_mut().init_rom_at_addr(0xd000, &rom[..]);
    }

    Ok(machine)
}
