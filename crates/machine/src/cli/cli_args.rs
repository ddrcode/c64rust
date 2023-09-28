use crate::machine::MachineConfig;
use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser, Debug, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub rom: Option<PathBuf>,

    #[arg(long)]
    pub ram: Option<PathBuf>,

    #[arg(long = "ram-file-addr")]
    pub ram_file_addr: Option<String>,

    #[arg(long = "ram-size", default_value_t = 65536)]
    #[serde(default = "Args::default_ram_size")]
    pub ram_size: usize,

    #[arg(short='a', long="start-addr", default_value_t=String::from("fce2"))]
    #[serde(default = "Args::default_start_addr")]
    pub start_addr: String,

    #[arg(short, long)]
    #[serde(default)]
    pub show_screen: bool,

    #[arg(long = "show-status")]
    #[serde(default)]
    pub show_status: bool,

    #[arg(short = 'd', long)]
    #[serde(default)]
    pub disassemble: bool,

    #[arg(long = "max-cycles")]
    pub max_cycles: Option<u128>,

    #[arg(long = "max-time")]
    pub max_time: Option<u64>,

    #[arg(long = "stop-on-addr")]
    pub stop_on_addr: Option<String>,

    #[arg(long = "stop-on-brk")]
    #[serde(default)]
    pub stop_on_brk: bool,

    #[arg(short = 'v', long)]
    #[serde(default)]
    pub verbose: bool,

    #[arg(long = "char-rom")]
    pub character_rom: Option<PathBuf>,

    #[arg(long = "profile")]
    pub profile: Option<PathBuf>,
}

impl From<&Args> for MachineConfig {
    fn from(args: &Args) -> Self {
        MachineConfig {
            ram_size: args.ram_size as usize,
            rom_size: 1 << 16,
            max_time: args.max_time,
            max_cycles: args.max_cycles,
            exit_on_addr: if let Some(str) = &args.stop_on_addr {
                Some(u16::from_str_radix(&str, 16).unwrap())
            } else {
                None
            },
            exit_on_brk: args.stop_on_brk,
            disassemble: args.disassemble,
            verbose: args.verbose,
        }
    }
}

fn if_else<T>(cond: bool, val1: T, val2: T) -> T {
    if cond {
        val1
    } else {
        val2
    }
}

fn val_or(val1: bool, val2: bool) -> bool {
    if val1 {
        val1
    } else {
        val2
    }
}

impl Args {
    pub fn default_start_addr() -> String {
        "fce2".to_string()
    }

    pub fn default_ram_size() -> usize {
        0xffff
    }

    pub fn merge(cli: &Args, file: &Args) -> Args {
        Args {
            rom: cli.rom.clone().or(file.rom.clone()),
            ram: cli.ram.clone().or(file.ram.clone()),
            ram_file_addr: cli.ram_file_addr.clone().or(file.ram_file_addr.clone()),
            ram_size: if_else(
                cli.ram_size != Args::default_ram_size(),
                cli.ram_size,
                file.ram_size,
            ),
            start_addr: if_else(
                cli.start_addr != Args::default_start_addr(),
                cli.start_addr.clone(),
                file.start_addr.clone(),
            ),
            show_screen: val_or(cli.show_screen, file.show_screen),
            show_status: val_or(cli.show_status, file.show_status),
            disassemble: val_or(cli.disassemble, file.disassemble),
            max_cycles: cli.max_cycles.or(file.max_cycles),
            max_time: cli.max_time.or(file.max_time),
            stop_on_addr: cli.stop_on_addr.clone().or(file.stop_on_addr.clone()),
            stop_on_brk: val_or(cli.stop_on_brk, file.stop_on_brk),
            verbose: val_or(cli.verbose, file.verbose),
            character_rom: cli.character_rom.clone().or(file.character_rom.clone()),
            profile: None,
        }
    }
}
