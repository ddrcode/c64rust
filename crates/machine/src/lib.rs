#[macro_use]
extern crate lazy_static;
extern crate colored;

pub mod cli;
pub mod client;
pub mod debugger;
mod error;
mod machine;
pub mod mos6502;
pub mod utils;
pub mod emulator;

pub use crate::error::MachineError;
pub use crate::machine::*;
