use crate::emulator::abstractions::{Pin, PinBuilder, PinDirection::*, Port};
use std::collections::HashMap;
use std::rc::Rc;

pub struct W65C02_Pins {
    pins: [Rc<Pin>; 40],
    pub data: Rc<Port<u8>>,
    pub addr: Rc<Port<u16>>,
}

impl W65C02_Pins {
    pub fn new() -> Self {
        let pins: Vec<Rc<Pin>> = PinBuilder::new(40)
            .set(1, "VP", Output)
            .set(2, "RDY", Input)
            .set(3, "PHI1O", Output)
            .set(4, "IRQ", Input)
            .set(5, "ML", Output)
            .set(6, "NMI", Input)
            .set(7, "SYNC", Output)
            .set(8, "VDD", Input)
            .set_range(9..=20, "A", 0, Output)
            .tri_state()
            .set(21, "VSS", Input)
            .set_range(22..=25, "A", 12, Output)
            .with_range(26..=33)
            .direction(Output)
            .group_dec("D", 7)
            .tri_state()
            .io()
            .set(34, "RW", Output)
            .tri_state()
            .set(35, "NC", Input)
            .set(36, "BE", Input)
            .set(37, "PHI2", Input)
            .set(38, "SO", Output)
            .set(39, "PHI2O", Output)
            .set(40, "RES", Input)
            .build()
            .iter()
            .map(move |pin| Rc::new(pin.clone()))
            .collect();

        let data = pins[25..33].to_vec();

        let addr = {
            let mut v1 = pins[8..20].to_vec();
            let mut v2 = pins[21..25].to_vec();
            v1.append(&mut v2);
            v1
        };

        W65C02_Pins {
            pins: pins
                .try_into()
                .unwrap_or_else(|_| panic!("Must have 40 pins")),
            data: Port::from_pins(8, data),
            addr: Port::from_pins(16, addr),
        }
    }

    pub fn by_id(&self, id: usize) -> Option<&Pin> {
        Some(&self.pins[id - 1])
    }

    pub fn by_name(&self, name: &str) -> Option<&Pin> {
        self.pins
            .iter()
            .find(|&pin| pin.name() == name)
            .map(|pin| pin.as_ref())
    }
}
