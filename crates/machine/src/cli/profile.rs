use std::{fs::read_to_string, path::PathBuf};

use anyhow::Result;
use serde_derive::Deserialize;

use crate::debugger::Breakpoint;

use super::Args;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub config: Args,
    pub debug: Option<DebuggerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DebuggerConfig {
    variable: Option<Vec<Var>>,
    breakpoint: Option<Vec<Breakpoint>>
}

#[derive(Debug, Deserialize)]
struct Var {
    pub name: String,
    pub address: u16,
}

pub fn get_profile_from_toml(file: PathBuf) -> Result<Profile> {
    let content = read_to_string(file)?;
    let profile: Profile = toml::from_str(&content)?;

    println!("{:?}", profile);

    Ok(profile)
}
