pub mod cli;
pub mod client;
pub mod debugger;
mod machine;
pub mod mos6502;
pub mod utils;
mod error;

pub use crate::machine::*;
pub use crate::error::MachineError;
