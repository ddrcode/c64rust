// #[macro_use]
extern crate colored;
extern crate cursive_hexview;
extern crate log;

mod gui;

use crate::gui::*;
use c64::{irq_loop, machine_loop, C64};
use clap::Parser;
use cursive::{event::Key, logger, menu, views::Canvas, CbSink, Cursive, CursiveRunnable};
use cursive_hexview::HexView;
use machine::{cli::*, utils::lock, Machine, MachineConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static IS_RUNNING: AtomicBool = AtomicBool::new(true);
const GUI_REFRESH: Duration = Duration::from_millis(500);

fn main() {
    // logger::set_internal_filter_level(LevelFilter::Warn);
    // logger::set_external_filter_level(LevelFilter::Debug);
    // set_filter_levels_from_env(LevelFilter::Debug);
    logger::init();
    colored::control::set_override(false);

    let c64 = Arc::new(Mutex::new(init_c64()));
    let c64_arc = Arc::clone(&c64);
    let mut siv = init_ui(c64);
    let mut threads = Vec::new();

    let mut start_thread = |cb: fn(c64: Arc<Mutex<C64>>, sink: CbSink, b: Arc<&AtomicBool>)| {
        let c64_t = c64_arc.clone();
        let sink_t = siv.cb_sink().clone();
        let b_t = Arc::new(&IS_RUNNING);
        let handle = thread::spawn(move || {
            cb(c64_t, sink_t, b_t);
        });
        threads.push(handle);
    };

    start_thread(|c64, _, _| machine_loop(c64));
    start_thread(|c64, _, _| irq_loop(c64));
    start_thread(|c64, sink, do_loop| {
        while do_loop.load(Ordering::Relaxed) {
            thread::sleep(GUI_REFRESH);
            let c = Arc::clone(&c64);
            sink.send(Box::new(|s| update_ui(s, c))).unwrap();
        }
    });

    siv.run();
    IS_RUNNING.store(false, Ordering::Relaxed);

    threads.into_iter().for_each(|t| {
        t.join().expect("Thread failed!");
    });
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
    siv.set_autorefresh(false);
    siv.set_autohide_menu(false);

    let quit_handler = {
        let arc = Arc::clone(&c64);
        move |s: &mut Cursive| {
            s.quit();
            lock(&arc).stop();
        }
    };

    let refresh_mem_handler = {
        let arc = Arc::clone(&c64);
        move |s: &mut Cursive| {
            s.call_on_name("memory", |view: &mut HexView| {
                view.set_data(lock(&arc).memory().mem(0).iter());
            });
        }
    };

    let debug_handler = {
        use machine::MachineStatus::*;
        let arc = Arc::clone(&c64);
        move |_s: &mut Cursive| {
            let mut c64 = lock(&arc);
            match c64.get_status() {
                Running => c64.debug(),
                Debug => c64.resume(),
                _ => (),
            };
        }
    };

    let next_handler = {
        let arc = Arc::clone(&c64);
        move |_s: &mut Cursive| {
            lock(&arc).next();
        }
    };

    let screen = main_screen(c64);

    siv.menubar()
        .add_subtree(
            "Machine",
            menu::Tree::new()
                .leaf("Pause", |_s| {})
                .leaf("Restart", |_s| {})
                .leaf("Stop interrupts", |_s| {}),
        )
        .add_subtree(
            "Monitor",
            menu::Tree::new()
                .leaf("Refresh [F5]", refresh_mem_handler.clone())
                .leaf("Go to address [F6]", |s| s.add_layer(address_dialog()))
                .delimiter()
                .leaf("Autorefresh: on", |_s| {}),
        )
        .add_subtree(
            "View",
            menu::Tree::new()
                .leaf("Hide memory view", |_s| ())
                .leaf("Hide processor status", |_s| ()),
        )
        .add_leaf("Quit", quit_handler.clone());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.add_global_callback(Key::F10, quit_handler);
    siv.add_global_callback(Key::F5, refresh_mem_handler);
    siv.add_global_callback(Key::F6, |s| s.add_layer(address_dialog()));
    siv.add_global_callback(Key::F7, debug_handler);
    siv.add_global_callback(Key::F8, next_handler);

    siv.add_layer(screen);
    siv.set_user_data(UIState::default());

    siv
}

fn set_theme(siv: &mut CursiveRunnable) {
    use cursive::theme::*;
    let mut theme = siv.current_theme().clone();
    theme.shadow = true;
    theme.borders = BorderStyle::None;
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::View] = Color::Rgb(0x9c, 0xa5, 0xb5);
    // theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    siv.set_theme(theme);
}

fn update_ui(s: &mut Cursive, c64: Arc<Mutex<C64>>) {
    let addr = if let Some(d) = s.user_data::<UIState>() {
        d.addr_from
    } else {
        0
    };

    s.call_on_name("memory", |view: &mut HexView| {
        let data = lock(&c64).memory().fragment(addr, addr + 200);
        view.set_start_addr(addr as usize);
        view.set_data(data.iter());
    });

    s.call_on_name("cpu", |view: &mut Canvas<CpuState>| {
        view.state_mut().state = lock(&c64).cpu().registers.to_string();
    });
}
