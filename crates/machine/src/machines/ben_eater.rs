use std::rc::Rc;

use crate::emulator::{
    abstractions::{Pin, Port},
    components::{HM62256BLogic, Oscilator, HM62256B},
    cpus::W65C02,
};

pub struct BenEaterMachine {
    clock: Oscilator,
    cpu: W65C02,
    ram: Rc<HM62256B<HM62256BLogic>>,
}

impl BenEaterMachine {
    pub fn new() -> Self {
        let clock = Oscilator::new(1000);
        let cpu = W65C02::new();
        let ram = HM62256B::new(HM62256BLogic::new());

        Pin::link(&clock.pin, cpu.pins.by_name("PHI_2").unwrap()).unwrap();
        Port::link(&cpu.pins.addr, &ram.pins.addr).unwrap();
        Port::link(&cpu.pins.data, &ram.pins.data).unwrap();
        Pin::link(
            &cpu.pins.by_name("RWB").unwrap(),
            &ram.pins.by_name("WE").unwrap(),
        )
        .unwrap();

        BenEaterMachine { clock, cpu, ram }
    }
}
