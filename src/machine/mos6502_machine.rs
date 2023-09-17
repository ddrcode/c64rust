use super::{
    impl_reg_setter, MOS6502Memory, Machine, MachineConfig, MachineEvents, Memory, RegSetter,
};
use crate::mos6510::{execute_operation, Operation, MOS6510};
use std::num::Wrapping;

pub struct MOS6502Machine {
    config: MachineConfig,
    mos6510: MOS6510,
    mem: Box<dyn Memory + Send>,
    events: MachineEvents,
}

impl MOS6502Machine {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        MOS6502Machine {
            config: config,
            mos6510: MOS6510::new(),
            mem: Box::new(MOS6502Memory::new(size)),
            events: MachineEvents { on_next: None },
        }
    }
}

impl_reg_setter!(MOS6502Machine);

impl Machine for MOS6502Machine {
    fn memory(&self) -> &Box<dyn Memory + Send + 'static> {
        &self.mem
    }

    fn memory_mut(&mut self) -> &mut Box<dyn Memory + Send + 'static> {
        &mut self.mem
    }

    fn cpu(&self) -> &MOS6510 {
        &self.mos6510
    }

    fn cpu_mut(&mut self) -> &mut MOS6510 {
        &mut self.mos6510
    }

    fn get_config(&self) -> &MachineConfig {
        &self.config
    }

    fn get_events(&self) -> &MachineEvents {
        &self.events
    }

    fn execute_operation(&mut self, op: &Operation) -> u8 {
        execute_operation(&op, self)
    }
}
