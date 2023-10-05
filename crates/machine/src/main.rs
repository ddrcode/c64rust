#[macro_use]
extern crate lazy_static;
extern crate colored;

pub mod cli;
pub mod client;
pub mod debugger;
pub mod error;
pub mod machine;
pub mod mos6502;
pub mod utils;
pub mod emulator;

use crate::cli::{create_machine_from_cli_args, Args};
use crate::client::{DirectClient, NonInteractiveClient};
use crate::machine::MOS6502Machine;
use anyhow::Result;
use clap::Parser;
use env_logger;

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let machine: MOS6502Machine = create_machine_from_cli_args()?;
    let mut client = DirectClient::new(machine);

    client.start_sync()?;

    if args.show_status {
        println!("{}", client.get_cpu_state()?);
    }

    client.stop()?;
    Ok(())
}
