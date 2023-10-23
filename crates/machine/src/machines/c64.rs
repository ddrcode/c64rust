use std::rc::Rc;

use crate::emulator::{cpus::MOS6510, abstractions::{Pin, PinDirection}};

pub struct C64 {
    clock: Rc<Pin>,
    cpu: Rc<MOS6510>,
}

impl C64 {
    pub fn new() -> Self {
        let clock = Pin::new(PinDirection::Output);
        let cpu = MOS6510::new();
        let _ = Pin::link(&clock, &cpu.pins.phi0);
        C64 { cpu, clock }
    }

    pub fn tick(&self) {
        self.clock.toggle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycles() {
        let c64 = C64::new();
        c64.tick();
        c64.tick();
        c64.tick();
        assert_eq!(3, c64.cpu.cycles());
    }
}
