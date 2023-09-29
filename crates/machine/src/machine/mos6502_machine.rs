use super::{
    impl_reg_setter, FromConfig, MOS6502Memory, Machine, MachineConfig, MachineStatus, Memory,
    RegSetter, Cycles
};
use crate::cli::{Profile, FromProfile};
use crate::debugger::DebugMachine;
use crate::mos6502::{execute_operation, Operation, MOS6502};
use std::num::Wrapping;

pub struct MOS6502Machine {
    config: MachineConfig,
    mos6502: MOS6502,
    mem: MOS6502Memory,
    status: MachineStatus,
    cycles: Cycles
}

impl MOS6502Machine {
    #[allow(dead_code)]
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        MOS6502Machine {
            config,
            mos6502: MOS6502::new(),
            mem: MOS6502Memory::new(size),
            status: MachineStatus::Stopped,
            cycles: 0
        }
    }
}

impl_reg_setter!(MOS6502Machine);

impl Machine for MOS6502Machine {
    type MemoryImpl = MOS6502Memory;

    fn memory(&self) -> &MOS6502Memory {
        &self.mem
    }

    fn memory_mut(&mut self) -> &mut MOS6502Memory {
        &mut self.mem
    }

    fn cpu(&self) -> &MOS6502 {
        &self.mos6502
    }

    fn cpu_mut(&mut self) -> &mut MOS6502 {
        &mut self.mos6502
    }

    fn get_config(&self) -> &MachineConfig {
        &self.config
    }

    fn get_status(&self) -> MachineStatus {
        self.status
    }

    fn set_status(&mut self, status: MachineStatus) {
        self.status = status;
    }

    fn execute_operation(&mut self, op: &Operation) -> u8 {
        execute_operation(&op, self)
    }

    fn get_cycles(&self) -> Cycles {
        self.cycles
    }

    fn advance_cycles(&mut self, cycles: u8) {
        self.cycles = self.cycles.wrapping_add(cycles.into());
    }
}

impl DebugMachine for MOS6502Machine {}

impl FromConfig for MOS6502Machine {
    fn from_config(config: MachineConfig) -> Self {
        let machine = MOS6502Machine::new(config);
        machine
    }
}

impl FromProfile for MOS6502Machine {
    fn from_profile(profile: &Profile) -> Self {
        MOS6502Machine::from_config(profile.into())
    }
}
