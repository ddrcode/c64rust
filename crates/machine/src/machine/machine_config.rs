pub struct MachineConfig {
    pub ram_size: usize,
    pub rom_size: usize,
    pub max_time: Option<u64>,
    pub max_cycles: Option<u128>,
    pub exit_on_addr: Option<u16>,
    pub exit_on_brk: bool,
    pub disassemble: bool,
    pub verbose: bool,
}

impl MachineConfig {
    #[allow(dead_code)]
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

pub trait FromConfig {
    fn from_config(config: MachineConfig) -> Self;
}


