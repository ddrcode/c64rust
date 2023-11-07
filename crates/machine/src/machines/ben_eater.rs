use std::rc::Rc;

use crate::emulator::{
    abstractions::{Addressable, Circuit, CircuitBuilder, Machine},
    components::{HM62256BLogic, Oscilator, HM62256B},
    cpus::W65C02,
    EmulatorError,
};

/// Implementation of a popular and simple W65C02-based breadboard computer designed by Ben Eater
/// Details: https://eater.net/6502
/// Work in progress (it is missing ROM and I/O, but works in the current form)
/// Address bus pin A15 is unconnected, as the machine has only 32kB of RAM (one address pin less)
/// In practice addresses pointing to the upper 32kB point in fact to the lower 32kB
pub struct BenEaterMachine {
    circuit: Rc<Circuit>,
}

impl BenEaterMachine {
    pub fn new() -> Result<Self, EmulatorError> {
        let clock = Oscilator::new(1000);
        let mut ram = HM62256B::new(HM62256BLogic::new());
        let cpu = W65C02::new();

        // Trick: forcess the address of reset vector. (should be handled by ROM)
        ram.logic.write_byte(0xfffc & 0x7fff, 0);
        ram.logic.write_byte(0xfffd & 0x7fff, 1);

        let circuit = CircuitBuilder::new()
            .add_component("X1", clock)
            .add_component("U1", cpu)
            .add_component("U6", ram)
            .link("X1", "OUT", "U1", "PHI2")
            .link("U1", "RW", "U6", "WE")
            .link_range("U1", "A", "U6", "A", 0..15)
            .link_to_vcc("U1", "NMI")
            .link_to_vcc("U1", "RDY")
            .link_to_vcc("U1", "BE")
            .build()?;

        Ok(BenEaterMachine { circuit })
    }
}

impl Machine for BenEaterMachine {
    fn start(&mut self) {
        self.reset();
    }

    fn stop(&mut self) {
        let _ = self.circuit.write_to_pin("U1", "VCC", false);
    }

    // W65C02 requires two cycles in high state on pin 40 (RST) to initialize or reset
    // Then, after start, first 7 cycles are initialization steps
    fn reset(&mut self) {
        let _ = self.circuit.write_to_pin("U1", "RST", true);
        self.step();
        self.step();
        let _ = self.circuit.write_to_pin("U1", "RST", false);
        for _ in 0..7 {
            self.step();
        }
    }

    fn step(&mut self) {
        self.circuit.with_pin("X1", "OUT", |pin| {
            let _ = pin.toggle();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_creation() {
        assert!(BenEaterMachine::new().is_ok());
    }
}
