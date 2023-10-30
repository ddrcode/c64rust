use std::rc::Rc;

use crate::{
    emulator::abstractions::{IPin, Pin, PinDirection, Port, Tickable, Ticker},
    machine::Cycles,
};

pub struct MOS6510Pins {
    pub address_bus: Port<u16>,
    pub data_bus: Port<u8>,
    pub processor_port: Port<u8>,
    pub irq: Rc<Pin>,
    pub nmi: Rc<Pin>,
    pub rdy: Rc<Pin>,
    pub phi0: Rc<Pin>,
    pub phi2: Rc<Pin>,
}

impl MOS6510Pins {
    pub fn new() -> Self {
        MOS6510Pins {
            address_bus: Port::new(16, PinDirection::Output),
            data_bus: Port::new(8, PinDirection::Output),
            processor_port: Port::new(6, PinDirection::Output),
            irq: Pin::input(),
            nmi: Pin::input(),
            rdy: Pin::input(),
            phi0: Pin::input(),
            phi2: Pin::output(),
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
        let cloned = cpu_rc.clone();
        // cpu_rc.pins.phi0.observe(move |_val| cloned.tick());

        cpu_rc
    }

    fn tick(&self) {
        self.pins.phi2.write(self.pins.phi0.state());
        self.ticker.tick();
    }

    pub fn cycles(&self) -> Cycles {
        self.ticker.cycles()
    }
}
