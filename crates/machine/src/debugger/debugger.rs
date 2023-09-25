use super::*;
use crate::mos6502::{ Operation };
use crate::machine:: { Machine, MachineStatus };

pub trait Debugger {
    fn debugger_state(&self) -> &DebuggerState;
    fn machine(&self) -> &dyn Machine;

    fn should_pause(&self, op: &Operation) -> bool {
        let m = self.machine();
        self.debugger_state().breakpoints.iter().any(|bp| bp.applies(op, m))
        
    }

}

pub trait DebugMachine: Debugger+Machine {
    fn start_debugging(&mut self) {
        self.set_status(MachineStatus::Debug);
    }
}
