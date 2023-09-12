use cursive::event::Key;

mod cli_args;
mod cli_utils;
mod gui;
mod machine;
mod c64;
mod mos6510;

// mod actions;
// mod views;
//
use clap::Parser;
use crate::gui::Screen;
use crate::c64::C64;
use crate::cli_args::Args;
use crate::cli_utils::get_file_as_byte_vec;
use crate::machine::{MachineConfig};

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

    let mut siv = cursive::default();
    
    // siv.add_layer(views::mainscreen());

    let mut screen = Screen::new();

    siv.menubar().add_leaf("Quit", |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    // siv.add_global_callback(Key::F5, |s| actions::execute_request(s));
    screen.content = c64.get_screen_memory();
    siv.add_layer(screen);


    siv.run();
}

