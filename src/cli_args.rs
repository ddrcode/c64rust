use clap::Parser;
use std::path::PathBuf;
use crate::c64::{C64Config};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub rom: Option<PathBuf>,

    #[arg(long)]
    pub ram: Option<PathBuf>,

    #[arg(long = "ram-file-addr", default_value_t = 0)]
    pub ram_file_addr: u16,

    #[arg(short='a', long="start-addr", default_value_t=String::from("fce2"))]
    pub start_addr: String,

    #[arg(short, long)]
    pub show_screen: bool,

    #[arg(long="show-status")]
    pub show_status: bool,

    #[arg(short = 'd', long)]
    pub disassemble: bool,

    #[arg(long = "max-cycles")]
    pub max_cycles: Option<u64>,

    #[arg(long = "max-time")]
    pub max_time: Option<u64>,

    #[arg(long = "stop-on-addr")]
    pub stop_on_addr: Option<String>,

    #[arg(short='v', long)]
    pub verbose: bool,
}

impl From<&Args> for C64Config {
    fn from(args: &Args) -> Self {
        C64Config {
            max_time: args.max_time,
            max_cycles: args.max_cycles,
            exit_on_addr: if let Some(str) = &args.stop_on_addr {
                Some(u16::from_str_radix(&str, 16).unwrap())
            } else {
                None
            },
            exit_on_op: None,
            disassemble: args.disassemble,
            verbose: args.verbose
        }
    }
}

