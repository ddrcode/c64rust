use super::*;
use crate::machine::{Machine, MachineStatus};
use crate::mos6502::{Mnemonic, Operation};

pub trait Debugger {
    type MachineImpl: Machine;

    fn debugger_state(&self) -> &DebuggerState;
    fn machine(&self) -> &Self::MachineImpl;

    fn should_pause(&self, op: &Operation) -> bool {
        let m = self.machine();
        self.debugger_state()
            .breakpoints
            .iter()
            .any(|bp| bp.applies(op, m))
    }
}

pub trait DebugMachine: Machine {
    fn start_debugging(&mut self) {
        self.set_status(MachineStatus::Debug);
    }
}
