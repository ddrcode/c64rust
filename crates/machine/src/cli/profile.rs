use serde_derive::Deserialize;

use crate::debugger::{Breakpoint, DebuggerState, Variable};
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

pub trait FromProfile {
    fn from_profile(profile: &Profile) -> Self;
}

impl From<&Var> for Variable {
    fn from(var: &Var) -> Self {
        Variable {
            name: var.name.clone(),
            addr: var.address,
            value: 0,
        }
    }
}

impl From<&DebuggerConfig> for DebuggerState {
    fn from(profile: &DebuggerConfig) -> Self {
        let mut state = DebuggerState::default();
        if let Some(v) = &profile.variable {
            state.variables = v.iter().map(|x| Variable::from(x)).collect();
        }
        if let Some(b) = &profile.breakpoint {
            state.breakpoints = b.to_vec();
        }
        state
    }
}
