#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate cursive_hexview;
extern crate log;

pub mod config;
pub mod gui;
pub mod messaging;
pub mod utils;

use crate::gui::*;
use crate::messaging::*;
use anyhow;
use c64::{C64Client, MachineState, C64};
use config::CONFIG;
use cursive::views::Dialog;
use log::LevelFilter;
use machine::client::ClientEvent;
use machine::{
    cli::create_machine_from_cli_args, client::NonInteractiveClient, utils::lock, MachineError,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const GUI_REFRESH: Duration = Duration::from_millis(50);

fn main() -> anyhow::Result<()> {
    colored::control::set_override(false);
    init_log();

    let client = init_client()?;
    let mut siv = init_ui(client.clone());

    let mut prev_state = MachineState::default();
    let mut runner = siv.runner();

    runner.refresh();
    init_breakpoints_view(&mut runner, &lock(&client).get_debugger_state().breakpoints);

    loop {
        runner.step();
        if !runner.is_running() {
            break;
        }
        if lock(&client).is_running() {
            let state = lock(&client).step();
            if state != prev_state {
                update_ui(&state, &mut runner);
                runner.refresh();
                prev_state = state;
            }
        }
        thread::sleep(GUI_REFRESH);
    }

    lock(&client.clone()).stop()?;
    Ok(())
}

fn init_client() -> anyhow::Result<Arc<Mutex<C64Client>>> {
    let mut c64_client = C64Client::new(create_machine_from_cli_args::<C64>()?);
    connect_client(&mut c64_client);
    let client = Arc::new(Mutex::new(c64_client));

    lock(&client).start().unwrap_or_else(handle_error);
    send_client_event(ClientEvent::SetObservedMemory(0..CONFIG.memory_view_size));

    Ok(client)
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

fn handle_error(err: MachineError) {
    log::error!("An error occured on emulator side: {}", err);
    Dialog::info("The emulator has failed!");
}
