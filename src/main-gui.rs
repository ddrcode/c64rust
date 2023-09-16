use cursive::event::Key;
use cursive::CursiveRunnable;

mod c64;
mod cli_args;
mod cli_utils;
mod gui;
mod machine;
mod mos6510;

// mod actions;
// mod views;
//
use crate::c64::{irq_loop, machine_loop, C64};
use crate::cli_args::Args;
use crate::cli_utils::get_file_as_byte_vec;
use crate::gui::Screen;
use crate::machine::{Machine, MachineConfig, Memory};
use clap::Parser;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn set_theme(siv: &mut CursiveRunnable) {
    let mut theme = siv.current_theme().clone();
    theme.shadow = false;
    // theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] =
        cursive::theme::Color::TerminalDefault;
    // theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    siv.set_theme(theme);
}

fn main() {
    let args = Args::parse();
    let c64 = Arc::new(Mutex::new(C64::new(MachineConfig::from(&args))));
    if let Some(rom_file) = args.rom {
        let rom = get_file_as_byte_vec(&rom_file);
        c64.lock().unwrap().memory_mut().init_rom(&rom[..]);
    }

    c64.lock().unwrap().power_on();

    if let Some(ram_file) = args.ram {
        let ram = get_file_as_byte_vec(&ram_file);
        let addr = u16::from_str_radix(&args.ram_file_addr, 16).unwrap();
        c64.lock().unwrap().memory_mut().write(addr, &ram[..]);
    }

    // c64.start();
    let machine_loop_c64 = Arc::clone(&c64);
    thread::spawn(move || {
        machine_loop(machine_loop_c64);
    });

    let irq_loop_c64 = Arc::clone(&c64);
    thread::spawn(move || {
        irq_loop(irq_loop_c64);
    });

    let mut siv = cursive::default();
    set_theme(&mut siv);

    siv.set_autorefresh(true);

    // siv.add_layer(views::mainscreen());

    let mut screen = Screen::new(c64);

    siv.menubar().add_leaf("Quit", |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.add_global_callback(Key::F10, |s| s.quit());
    // siv.add_global_callback(Key::F5, |s| actions::execute_request(s));
    siv.add_layer(screen);

    siv.run();
}
