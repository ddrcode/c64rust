use anyhow::Result;
use serde_derive::Deserialize;

use crate::debugger::Breakpoint;
use crate::machine::MachineConfig;

use super::Args;

#[derive(Debug, Deserialize, Default)]
pub struct Profile {
    pub config: Args,
    pub debug: Option<DebuggerConfig>,
}

impl Profile {
    pub fn from_args(args: &Args) -> Self {
        Profile {
            config: args.clone(),
            debug: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DebuggerConfig {
    variable: Option<Vec<Var>>,
    breakpoint: Option<Vec<Breakpoint>>,
}

#[derive(Debug, Deserialize)]
struct Var {
    pub name: String,
    pub address: u16,
}

impl From<&Profile> for MachineConfig {
    fn from(profile: &Profile) -> Self {
        MachineConfig::from(&profile.config)
    }
}
