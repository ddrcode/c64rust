use super::*;
use crate::machine::Addr;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub variables: Vec<Variable>,
    pub irq_on: bool,
    pub nmi_on: bool,
    pub observed_mem: Range<Addr>,
}
