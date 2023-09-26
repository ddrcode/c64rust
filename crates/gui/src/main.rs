// #[macro_use]
extern crate colored;
extern crate cursive_hexview;
extern crate log;

mod gui;

use crate::gui::*;
use c64::{C64Client, MachineState, C64};
use clap::Parser;
use cursive::{
    event::Key,
    logger, menu,
    views::{Canvas, TextView},
    Cursive, CursiveRunnable,
};
use cursive_hexview::HexView;
use log::LevelFilter;
use machine::{
    cli::*,
    client::{Client, ClientError, InteractiveClient, NonInteractiveClient},
    utils::lock,
    Machine, MachineConfig,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const GUI_REFRESH: Duration = Duration::from_millis(50);

fn main() -> Result<(), ClientError> {
    colored::control::set_override(false);
    init_log();

    let mut c64_client = C64Client::new(init_c64());
    let mut siv = init_ui(c64_client.mutex());

    c64_client.start()?;

    let mut prev_state = MachineState::default();
    let mut runner = siv.runner();
    runner.refresh();
    loop {
        runner.step();
        if !runner.is_running() {
            break;
        }
        let state = c64_client.step();
        if state != prev_state {
            update_ui(&state, &mut runner);
            runner.refresh();
            prev_state = state;
        }
        handle_user_data(&mut c64_client, &mut runner);
        thread::sleep(GUI_REFRESH);
    }

    c64_client.stop()
}

fn handle_user_data(client: &mut C64Client, s: &mut Cursive) {
    if let Some(ud) = s.user_data::<UIState>() {
        client.debugger_state.observed_mem = ud.addr_from..(ud.addr_from + 200);
    }
}

fn update_ui(state: &MachineState, s: &mut Cursive) {
    let addr = s.user_data::<UIState>().map_or(0, |data| data.addr_from);
    let screen = state.screen.clone();

    s.call_on_name("memory", |view: &mut HexView| {
        view.config_mut().start_addr = addr as usize;
        view.set_data(state.memory_slice.iter());
    });

    s.call_on_name("cpu", |view: &mut Canvas<CpuState>| {
        view.state_mut().state = state.registers.to_string();
    });

    s.call_on_name("machine_screen", move |view: &mut MachineScreen| {
        view.set_state(screen);
    });

    update_asm_view(s, &state.last_op);
}

fn init_c64() -> C64 {
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
    siv.add_global_callback(Key::F6, |s| s.add_layer(address_dialog()));
    siv.add_global_callback(Key::F7, debug_handler);
    siv.add_global_callback(Key::F8, next_handler);
    siv.add_global_callback(Key::F2, cursive::Cursive::toggle_debug_console);

    siv.add_layer(screen);
    siv.set_user_data(UIState::new());

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

fn init_log() {
    cursive::logger::init();
    match std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .as_ref()
    {
        "trace" => log::set_max_level(LevelFilter::Trace),
        "debug" => log::set_max_level(LevelFilter::Debug),
        "info" => log::set_max_level(LevelFilter::Info),
        "warn" => log::set_max_level(LevelFilter::Warn),
        "error" => log::set_max_level(LevelFilter::Error),
        _ => log::set_max_level(LevelFilter::Off),
    };
}
