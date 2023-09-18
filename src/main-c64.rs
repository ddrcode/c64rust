extern crate colored;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod c64;
mod cli_args;
mod machine;
mod mos6510;

use crate::c64::{machine_loop, C64};
use crate::cli_args::Args;
use crate::machine::{Machine, MachineConfig};
use std::sync::{Arc, Mutex};

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
        c64.memory_mut().init_rom(&rom[..]);
    }

    c64.power_on();

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        c64.memory_mut().write(addr, &ram[..]);
    }

    // c64.machine.start();
    let arc = Arc::new(Mutex::new(c64));
    let arc_loop: Arc<Mutex<dyn Machine>> = arc.clone();
    machine_loop(arc_loop);

    if args.show_status {
        println!("{}", arc.lock().unwrap().cpu().registers);
    }

    if args.show_screen {
        arc.lock().unwrap().print_screen();
    }

    arc.lock().unwrap().memory().dump(0, 0x30);
}
