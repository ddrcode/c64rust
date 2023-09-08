extern crate colored;

use std::env;
use std::fs::File;
use std::io::Read;
use clap::Parser;
use std::path::PathBuf;

mod c64;
mod mos6510;

#[cfg(test)]
mod tests;

use crate::c64::{ C64, C64Config };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom: Option<PathBuf>,

    #[arg(long)]
    ram: Option<PathBuf>,

    #[arg(long="ram-file-addr", default_value_t=0)]
    ram_file_addr: u16,

    #[arg(short='a', long="start-addr", default_value_t=String::from("fce2"))]
    start_addr: String,

    #[arg(short, long)]
    show_screen: bool,

    #[arg(short='d', long)]
    disassemble: bool,

    #[arg(long="max-cycles")]
    max_cycles: Option<u64>,

    #[arg(long="max-time")]
    max_time: Option<u64>,

    #[arg(long="stop-on-addr")]
    stop_on_addr: Option<String>
}

impl From<&Args> for C64Config {
    fn from(args: &Args) -> Self {
        C64Config {
            max_time: args.max_time,
            max_cycles: args.max_cycles,
            exit_on_addr: if let Some(str)=&args.stop_on_addr {
                Some(u16::from_str_radix(&str, 16).unwrap())
            } else { None },
            exit_on_op: None,
            disassemble: args.disassemble
        }
    }
}

fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() {
    let args = Args::parse();
    let mut c64 = C64::new(C64Config::from(&args));

    if let Some(rom_file)=args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        c64.mem.init_rom(&rom[..]);
    }

    c64.power_on();

    if let Some(ram_file)=args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        c64.mem.write(args.ram_file_addr, &ram[..]);
    }

    c64.run(u16::from_str_radix(&args.start_addr, 16).unwrap()); // start KERNAL

    if args.show_screen {
        c64.print_screen();
    }
}
