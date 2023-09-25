use super::*;

#[derive(Debug, Default, Clone)]
pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub variables: Vec<Variable>,
    pub irq_on: bool,
    pub nmi_on: bool,
}
