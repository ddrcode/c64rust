use crate::mos6510::Mnemonic;

pub struct MachineConfig {
    pub ram_size: usize,
    pub rom_size: usize,
    pub max_time: Option<u64>,
    pub max_cycles: Option<u64>,
    pub exit_on_addr: Option<u16>,
    pub exit_on_brk: bool,
    pub disassemble: bool,
    pub verbose: bool,
}

impl MachineConfig {
    pub fn new() -> Self {
        MachineConfig {
            ram_size: 1 << 16,
            rom_size: 1 << 16,
            max_time: None,
            max_cycles: None,
            exit_on_addr: None,
            exit_on_brk: false,
            disassemble: false,
            verbose: false,
        }
    }
}
