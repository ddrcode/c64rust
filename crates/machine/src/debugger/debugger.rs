use super::*;
use crate::machine::{Machine, MachineStatus};
use crate::mos6502::{Mnemonic, Operation};

pub trait Debugger {
    fn debugger_state(&self) -> &DebuggerState;
    fn machine(&self) -> &dyn Machine;

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

    fn pre_next(&mut self, op: &Operation) {
        if self.get_config().disassemble {
            self.print_op(&op);
        }

        if self.get_config().exit_on_brk && matches!(op.def.mnemonic, Mnemonic::BRK) {
            self.stop();
        }
    }

    fn print_op(&self, op: &Operation) {
        let addr = self.PC().wrapping_sub(op.def.len() as u16);
        let val = match op.def.len() {
            2 => format!("{:02x}   ", self.get_byte(addr + 1)),
            3 => format!(
                "{:02x} {:02x}",
                self.get_byte(addr + 1),
                self.get_byte(addr + 2)
            ),
            _ => String::from("     "),
        };
        print!("{:04x}: {:02x} {} | {}", addr, op.def.opcode, val, op);
        if self.get_config().verbose {
            print!(
                "{}|  {}",
                " ".repeat(13 - op.to_string().len()),
                self.cpu().registers,
            );
        }
        println!();
    }
}
