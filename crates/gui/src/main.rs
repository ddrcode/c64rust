#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate cursive_hexview;
extern crate log;

mod gui;
mod utils;

use crate::gui::*;
use c64::{C64Client, MachineState, C64};
use clap::Parser;
use cursive::{
    event::{Event, Key},
    logger, menu,
    view::ViewWrapper,
    views::{self, NamedView, OnEventView, ScrollView},
    views::{Canvas, Dialog, HideableView, PaddedView, ResizedView, TextView},
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

    let c64_client = C64Client::new(init_c64());
    let client = Arc::new(Mutex::new(c64_client));
    let mut siv = init_ui(client.clone());

    lock(&client).start().unwrap_or_else(handle_error);

    let mut prev_state = MachineState::default();
    let mut runner = siv.runner();
    runner.refresh();
    loop {
        runner.step();
        if !runner.is_running() {
            break;
        }
        let state = lock(&client).step();
        if state != prev_state {
            update_ui(&state, &mut runner);
            runner.refresh();
            prev_state = state;
        }
        handle_user_data(client.clone(), &mut runner);
        thread::sleep(GUI_REFRESH);
    }

    lock(&client.clone()).stop()
}

fn handle_user_data(client: Arc<Mutex<C64Client>>, s: &mut Cursive) {
    if let Some(ud) = s.user_data::<UIState>() {
        lock(&client).debugger_state.observed_mem = ud.addr_from..(ud.addr_from + 200);
        ud.key
            .as_ref()
            .map(|key| lock(&client).send_key(key.clone()));
        ud.key = None;
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

fn init_ui(client: Arc<Mutex<C64Client>>) -> CursiveRunnable {
    let mut siv = cursive::default();
    set_theme(&mut siv);
    siv.set_autorefresh(false);
    siv.set_autohide_menu(false);

    let quit_handler = {
        let arc = client.clone();
        move |s: &mut Cursive| {
            s.quit();
            lock(&arc).stop().unwrap_or_else(handle_error);
        }
    };

    let debug_handler = {
        use machine::MachineStatus::*;
        let arc = Arc::clone(&client);
        move |_s: &mut Cursive| {
            let mut c64 = lock(&arc);
            (match c64.get_status() {
                Running => c64.pause(),
                Debug => c64.resume(),
                _ => Ok(()),
            })
            .unwrap_or_else(handle_error);
        }
    };

    let next_handler = {
        let arc = client.clone();
        move |_s: &mut Cursive| {
            lock(&arc).next().unwrap_or_else(|err| {
                handle_error(err);
                false
            });
        }
    };

    let screen = main_screen();

    type AsmIsEasierThanThis =
        ResizedView<PaddedView<OnEventView<ScrollView<NamedView<TextView>>>>>;

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
            menu::Tree::new().leaf("Go to address [F6]", |s| s.add_layer(address_dialog())),
        )
        .add_subtree(
            "View",
            menu::Tree::new()
                .leaf("Toggle memory view", |_s| ())
                .leaf("Toggle processor status", |_s| ())
                .leaf(
                    "Toggle disassembly view [F2]",
                    create_toggle_handler::<AsmIsEasierThanThis>("asm_wrapper"),
                ),
        )
        .add_leaf("Quit", quit_handler.clone());

    siv.add_global_callback(Key::F9, |s| s.select_menubar());
    siv.add_global_callback(Key::F10, quit_handler);
    siv.add_global_callback(Key::F6, |s| s.add_layer(address_dialog()));
    siv.add_global_callback(Key::F7, debug_handler);
    siv.add_global_callback(Key::F8, next_handler);
    siv.add_global_callback(Event::Char('`'), cursive::Cursive::toggle_debug_console);
    siv.add_global_callback(
        Key::F2,
        create_toggle_handler::<AsmIsEasierThanThis>("asm_wrapper"),
    );

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

fn handle_error(err: ClientError) {
    log::error!("An error occured on emulator side: {}", err);
    Dialog::info("The emulator has failed!");
}

fn create_toggle_handler<V: ViewWrapper>(name: &str) -> impl Fn(&mut Cursive) + '_ {
    |s| {
        log::error!("calling!");
        s.call_on_name(name, |view: &mut HideableView<V>| {
            let visible = view.is_visible();
            view.set_visible(!visible);
        });
    }
}

