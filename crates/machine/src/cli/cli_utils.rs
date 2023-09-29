use crate::machine::{Machine, Memory};
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::PathBuf;

use super::{Args, FromProfile, Profile};
use anyhow::Result;
use clap::Parser;
use toml;

pub fn get_file_as_byte_vec(filename: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_args_from_toml(file: PathBuf) -> Result<Args> {
    let content = read_to_string(file)?;
    let args: Args = toml::from_str(&content)?;
    Ok(args)
}

pub fn get_profile(args: Args) -> Result<Profile> {
    if args.profile.is_none() {
        return Ok(Profile::from_args(&args));
    }
    let mut profile = get_profile_from_toml(args.profile.clone().unwrap())?;
    profile.config = Args::merge(&args, &profile.config);
    Ok(profile)
}

pub fn get_profile_from_toml(file: PathBuf) -> Result<Profile> {
    let content = read_to_string(file)?;
    let profile: Profile = toml::from_str(&content)?;
    Ok(profile)
}

pub fn create_machine_from_cli_args<M>() -> Result<M>
where
    M: FromProfile + Machine,
{
    let args = Args::parse();
    let profile = get_profile(args)?;

    let mut machine = M::from_profile(&profile);

    if let Some(rom_file) = profile.config.rom {
        let rom = get_file_as_byte_vec(&rom_file)?;
        machine.memory_mut().init_rom(&rom[..]);
    }

    if let Some(ram_file) = profile.config.ram {
        let ram = get_file_as_byte_vec(&ram_file)?;
        let addr = u16::from_str_radix(
            &profile
                .config
                .ram_file_addr
                .ok_or(anyhow::Error::msg("couldn't parse ram-file-addr"))?,
            16,
        )?;
        machine.memory_mut().write(addr, &ram[..]);
    }

    if let Some(character_rom) = profile.config.character_rom {
        let rom = get_file_as_byte_vec(&character_rom)?;
        machine.memory_mut().init_rom_at_addr(0xd000, &rom[..]);
    }

    Ok(machine)
}
