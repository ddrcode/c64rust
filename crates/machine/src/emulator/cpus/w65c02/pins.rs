use crate::emulator::abstractions::{IPin, Pin, PinBuilder, PinDirection::*, Port};
use std::collections::HashMap;
use std::rc::Rc;

pub struct W65C02_Pins {
    pins: [Rc<Pin>; 40],
    pins_map: HashMap<String, Rc<Pin>>,
    pub data: Rc<Port<u8>>,
    pub addr: Rc<Port<u16>>,
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
            .group_dec("D", 7)
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

        let mut pins_map: HashMap<String, Rc<Pin>> = HashMap::with_capacity(40);
        pins.iter().for_each(|pin| {
            pins_map.insert(pin.name().unwrap(), Rc::clone(pin));
        });

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
            pins_map,
            data: Port::from_pins(8, data),
            addr: Port::from_pins(16, addr),
        }
    }

    pub fn by_id(&self, id: usize) -> Option<&Rc<Pin>> {
        Some(&self.pins[id - 1])
    }

    pub fn by_name(&self, name: &str) -> Option<&Rc<Pin>> {
        self.pins_map.get(name)
    }
}
