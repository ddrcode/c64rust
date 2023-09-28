#[macro_use]
extern crate lazy_static;
extern crate colored;

use clap::Parser;
use machine::cli::create_machine_from_cli_args;

mod c64;
mod client;
mod key_utils;

use crate::c64::C64;
use crate::client::C64Client;
use anyhow;
use machine::{cli::Args,client::NonInteractiveClient};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let c64: C64 = create_machine_from_cli_args()?;
    let mut client = C64Client::new(c64);
    client.start_sync()?;

    if args.show_status {
        println!("{}", client.get_cpu_state().unwrap());
    }

    if args.show_screen {
        client.mutex().lock().unwrap().print_screen();
    }

    client.stop()?;
    Ok(())
}
