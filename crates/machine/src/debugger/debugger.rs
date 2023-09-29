use super::*;
use crate::machine::{Machine, MachineStatus};
use crate::mos6502::{Operation};

pub trait MachineObserver {
    fn on_next(&mut self, op: &Operation);
}

pub trait Debugger {
    type MachineImpl: Machine;

    fn debugger_state(&self) -> &DebuggerState;
    fn debugger_state_mut(&mut self) -> &mut DebuggerState;
    fn machine(&self) -> &Self::MachineImpl;

    fn should_pause(&self, op: &Operation) -> bool {
        let m = self.machine();
        self.debugger_state()
            .breakpoints
            .iter()
            .any(|bp| bp.applies(op, m))
    }

    fn update_debugger_state(&mut self) {
        let vars = self.debugger_state().variables.iter().map(|var| Variable {
            value: self.machine().get_byte(var.addr),
            ..var.clone()
        }).collect();
        self.debugger_state_mut().variables = vars;
    }
}

pub trait DebugMachine: Machine {
    fn start_debugging(&mut self) {
        self.set_status(MachineStatus::Debug);
    }
}
