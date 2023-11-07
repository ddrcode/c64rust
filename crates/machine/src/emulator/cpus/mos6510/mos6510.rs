use std::rc::Rc;

use crate::{
    emulator::abstractions::{Pin, PinDirection, Port, Tickable, Ticker},
    machine::Cycles,
};

pub struct MOS6510Pins {
    pub address_bus: Rc<Port<u16>>,
    pub data_bus: Rc<Port<u8>>,
    pub processor_port: Rc<Port<u8>>,
    pub irq: Pin,
    pub nmi: Pin,
    pub rdy: Pin,
    pub phi0: Pin,
    pub phi2: Pin,
}

impl MOS6510Pins {
    pub fn new() -> Self {
        MOS6510Pins {
            address_bus: Port::new("A", 16, PinDirection::Output),
            data_bus: Port::new("D", 8, PinDirection::Output),
            processor_port: Port::new("P", 6, PinDirection::Output),
            irq: Pin::input("IRQ"),
            nmi: Pin::input("NMI"),
            rdy: Pin::input("RDY"),
            phi0: Pin::input("PHI0"),
            phi2: Pin::output("PHI2"),
        }
    }
}

pub struct MOS6510 {
    pub pins: MOS6510Pins,
    ticker: Ticker,
}

impl MOS6510 {
    pub fn new() -> Rc<Self> {
        let cpu = MOS6510 {
            pins: MOS6510Pins::new(),
            ticker: Ticker::new(),
        };

        let cpu_rc = Rc::new(cpu);
        // let cloned = cpu_rc.clone();
        // cpu_rc.pins.phi0.observe(move |_val| cloned.tick());

        cpu_rc
    }

    fn tick(&self) {
        self.pins.phi2.write(self.pins.phi0.state()).unwrap();
        self.ticker.tick();
    }

    pub fn cycles(&self) -> Cycles {
        self.ticker.cycles()
    }
}
