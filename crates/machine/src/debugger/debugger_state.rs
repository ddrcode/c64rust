use super::*;
use crate::machine::Addr;
use std::ops::Range;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub variables: Vec<Variable>,
    pub irq_on: bool,
    pub nmi_on: bool,
    pub observed_mem: Range<Addr>,
}

impl DebuggerState {
    pub fn set_observed_mem(&mut self, from: Addr, to: Addr) {
        self.observed_mem.start = from;
        self.observed_mem.end = to;
    }

    pub fn upsert_variable(&mut self, var: Variable) {
        if let Some(idx) = self.variables.iter().position(|v| v.name == var.name) {
            self.variables[idx] = var;
        } else {
            self.variables.push(var);
        }
    }

    pub fn remove_breakpoint(&mut self, bp: &Breakpoint) {
        log::info!("Calling remove_breakpoint with {:?}", bp);
        self.breakpoints.retain(|b| *b==*bp);
    }

    pub fn add_breakpoint(&mut self, bp: &Breakpoint) {
        if !self.breakpoints.contains(bp) {
            self.breakpoints.push(*bp);
        }
    }
}
