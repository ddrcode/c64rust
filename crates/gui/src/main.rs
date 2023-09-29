#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate cursive_hexview;
extern crate log;

mod gui;
mod utils;

use crate::gui::*;
use anyhow;
use c64::{C64Client, MachineState, C64};
use cursive::{views::Dialog, Cursive};
use log::LevelFilter;
use machine::{
    cli::create_machine_from_cli_args,
    client::{InteractiveClient, NonInteractiveClient},
    utils::lock,
    MachineError,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const GUI_REFRESH: Duration = Duration::from_millis(50);

fn main() -> anyhow::Result<()> {
    colored::control::set_override(false);
    init_log();

    let c64_client = C64Client::new(create_machine_from_cli_args::<C64>()?);
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

    lock(&client.clone()).stop()?;
    Ok(())
}

fn handle_user_data(client: Arc<Mutex<C64Client>>, s: &mut Cursive) {
    if let Some(ud) = s.user_data::<UIState>() {
        let mut state = lock(&client).get_debugger_state();
        state.observed_mem = ud.addr_from..(ud.addr_from + 200);
        lock(&client).set_debugger_state(state);
        ud.key
            .as_ref()
            .map(|key| lock(&client).send_key(key.clone()));
        ud.key = None;
    }
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
