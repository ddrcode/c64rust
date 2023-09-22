extern crate cursive_hexview;

mod c64;
mod cli_args;
mod cli_utils;
mod gui;
mod machine;
mod mos6510;

use crate::c64::{irq_loop, machine_loop, C64};
use crate::cli_args::Args;
use crate::cli_utils::get_file_as_byte_vec;
use crate::gui::{main_screen};
use crate::machine::{Machine, MachineConfig};
use clap::Parser;
use cursive::{event::Key, logger, menu, CbSink, Cursive, CursiveRunnable};
use cursive_hexview::HexView;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// #[macro_use]
extern crate log;

use log::LevelFilter;

static IS_RUNNING: AtomicBool = AtomicBool::new(true);
const GUI_REFRESH: Duration = Duration::from_millis(500);


fn set_theme(siv: &mut CursiveRunnable) {
    use cursive::theme::*;
    let mut theme = siv.current_theme().clone();
    theme.shadow = true;
    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] =
        cursive::theme::Color::TerminalDefault;
    theme.palette[cursive::theme::PaletteColor::View] =
        cursive::theme::Color::Dark(BaseColor::Blue);
    // theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    siv.set_theme(theme);
}

fn main() {
    // logger::set_internal_filter_level(LevelFilter::Warn);
    // logger::set_external_filter_level(LevelFilter::Debug);
    // set_filter_levels_from_env(LevelFilter::Debug);
    logger::init();

    let c64 = Arc::new(Mutex::new(init_c64()));
    let c64_arc = Arc::clone(&c64);
    let mut siv = init_ui(c64);

    let start_thread = |cb: fn(c64: Arc<Mutex<C64>>, sink: CbSink, b: Arc<&AtomicBool>)| {
        let c64_t = c64_arc.clone();
        let sink_t = siv.cb_sink().clone();
        let b_t = Arc::new(&IS_RUNNING);
        thread::spawn(move || {
            cb(c64_t, sink_t, b_t);
        })
    };

    let machine_thread = start_thread(|c64, _, _| machine_loop(c64));
    let irq_thread = start_thread(|c64, _, _| irq_loop(c64));
    let siv_thread = start_thread(|c64, sink, do_loop| {
        let clj = move |s: &mut Cursive| {
            s.call_on_name("memory", |view: &mut HexView| {
                view.set_data(c64.lock().unwrap().memory().mem(0).iter());
            });
        };
        while do_loop.load(Ordering::Relaxed) {
            thread::sleep(GUI_REFRESH);
            sink.send(Box::new(clj.clone())).unwrap();
        }
    });

    siv.run();
    IS_RUNNING.store(false, Ordering::Relaxed);

    let _ = machine_thread.join();
    let _ = irq_thread.join();
    let _ = siv_thread.join();
}

fn init_c64() -> C64 {
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

    c64
}

fn init_ui(c64: Arc<Mutex<C64>>) -> CursiveRunnable {
    let mut siv = cursive::default();
    set_theme(&mut siv);
    // siv.set_autorefresh(true);
    siv.set_autohide_menu(false);
    siv.set_fps(15);

    let quit_handler = {
        let arc = Arc::clone(&c64);
        move |s: &mut Cursive| {
            s.quit();
            arc.lock().unwrap().stop();
        }
    };

    let refresh_mem_handler = {
        let arc = Arc::clone(&c64);
        move |s: &mut Cursive| {
            s.call_on_name("memory", |view: &mut HexView| {
                view.set_data(arc.lock().unwrap().memory().mem(0).iter());
            });
        }
    };

    let screen = main_screen(c64);

    siv.menubar()
        .add_subtree(
            "Machine",
            menu::Tree::new()
                .leaf("Pause", |s| {})
                .leaf("Restart", |s| {})
                .leaf("Stop interrupts", |s| {}),
        )
        .add_subtree(
            "Memory",
            menu::Tree::new()
                .leaf("Refresh [F5]", refresh_mem_handler.clone())
                .leaf("Go to address [F6]", |s| {}),
        )
        .add_leaf("Quit [F10]", quit_handler.clone());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.add_global_callback(Key::F10, quit_handler);
    siv.add_global_callback(Key::F5, refresh_mem_handler);

    siv.add_layer(screen);

    siv
}
