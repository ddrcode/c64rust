use std::rc::Rc;

use crate::emulator::{
    abstractions::{Addressable, Circuit, CircuitBuilder, Machine},
    components::{HM62256BLogic, Oscilator, HM62256B},
    cpus::W65C02,
};

/// Implementation of a popular and simple W65C02-based breadboard computer designed by Ben Eater
/// Details: https://eater.net/6502
/// Work in progress (it is missing ROM and I/O, but works in the current form)
/// Address bus pin A16 is unconnected, as the machine has only 32kB of RAM
/// In practice addresses pointing to the upper 32kB point in fact to the lower 32kB
pub struct BenEaterMachine {
    circuit: Rc<Circuit>,
}

impl BenEaterMachine {
    pub fn new() -> Self {
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
            .link_range("U1", "A", "U6", "A", 1..16)
            .link_to_vcc("U1", "NMI")
            .link_to_vcc("U1", "RDY")
            .build()
            .unwrap();

        BenEaterMachine { circuit }
    }
}

impl Machine for BenEaterMachine {
    fn start(&mut self) {
        self.reset();
    }

    fn stop(&mut self) {
        self.circuit.with_pin("U1", "VCC", |pin| pin.set_low());
    }

    // W65C02 requires two cycles in high state on pin 40 (RST) to initialize or reset
    // Then, after start, first 7 cycles are initialization steps
    fn reset(&mut self) {
        self.circuit.with_pin("U1", "RST", |pin| pin.set_high());
        self.step();
        self.step();
        self.circuit.with_pin("U1", "RST", |pin| pin.set_low());
        for _ in 0..7 {
            self.step();
        }
    }

    fn step(&mut self) {
        self.circuit.with_pin("X1", "OUT", |pin| pin.toggle());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_creation() {
        let _be = BenEaterMachine::new();
    }
}
