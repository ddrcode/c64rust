use crate::emulator::EmulatorError;

use super::AddressResolver;

pub type CPUCycles = u64;

pub trait CPU {
    fn cycles(&self) -> CPUCycles;
    fn advance_cycles(&mut self) -> Result<(), EmulatorError>;
}

pub trait Machine : AddressResolver {
    type CPU: CPU;

    fn cpu(&self) -> &Self::CPU;
    fn cpu_mut(&mut self) -> &mut Self::CPU;

    fn start(&mut self);
    fn stop(&mut self);
    fn restart(&mut self);
    fn step(&mut self);
}



