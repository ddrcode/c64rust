use crate::emulator::abstractions::{Pin, PinBuilder, PinDirection::*};
use std::rc::Rc;

pub struct W65C02_Pins {
    pins: [Rc<Pin>; 40],
}

impl W65C02_Pins {
    pub fn new() -> Self {
        let pins = PinBuilder::new(40)
            .set(1, "VPB", Output)
            .set(2, "RDY", Input)
            .set(3, "PHI1O", Output)
            .set(4, "IRQB", Input)
            .set(5, "MLB", Output)
            .set(6, "NMIB", Input)
            .set(7, "SYNC", Output)
            .set(8, "VDD", Input)
            .set_range(9..=20, "A", 0, Output)
            .tri_state()
            .set(21, "VSS", Input)
            .set_range(22..=25, "A", 12, Output)
            .with_range(26..=33)
            .direction(Output)
            .name_prefix_dec("D", 7)
            .tri_state()
            .io()
            .set(34, "RWB", Output)
            .tri_state()
            .set(35, "NC", Input)
            .set(36, "BE", Input)
            .set(37, "PHI2", Input)
            .set(38, "SOB", Output)
            .set(39, "PHI2O", Output)
            .set(40, "RESB", Input)
            .build();

        W65C02_Pins {
            pins: pins
                .try_into()
                .unwrap_or_else(|_| panic!("Must have 40 pins")),
        }
    }

    pub fn by_id(&self, id: usize) -> Rc<Pin> {
        self.pins[id-1]
    }

    pub fn by_name(&self, name: &str) -> Rc<Pin> {

    }
}
