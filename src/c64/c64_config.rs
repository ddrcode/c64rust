use crate::mos6510::Mnemonic;

pub struct C64Config {
    pub ram_size: usize,
    pub rom_size: usize,
    pub max_time: Option<u64>,
    pub max_cycles: Option<u64>,
    pub exit_on_addr: Option<u16>,
    pub exit_on_op: Option<Mnemonic>,
    pub disassemble: bool,
    pub verbose: bool,
}

impl C64Config {
    pub fn new() -> Self {
        C64Config {
            ram_size: 1 << 16,
            rom_size: 1 << 16,
            max_time: None,
            max_cycles: None,
            exit_on_addr: None,
            exit_on_op: None,
            disassemble: false,
            verbose: false,
        }
    }
}
