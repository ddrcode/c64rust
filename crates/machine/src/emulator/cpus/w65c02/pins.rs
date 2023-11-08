use crate::emulator::abstractions::{Pin, PinBuilder, PinDirection::*, Pins, Port};
use std::rc::Rc;

/// ```text
///                W65C02
///            --------------
///     VP <-- |  1      40 | <-- RES
///   /RDY --> |  2      39 | --> PHI2O
///  PHI1O <-- |  3      38 | <-- SO
///    IRQ --> |  4      37 | <-- PHI2
///     ML <-- |  5     @36 | <-- BE
///    NMI --> |  6      35 | --- NC
///   SYNC <-- |  7     *34 | --> RW
///    VDD --> |  8     *33 | <-> D0
///     A0 <-- |  9*    *32 | <-> D1
///     A1 <-- | 10*    *31 | <-> D2
///     A2 <-- | 11*    *30 | <-> D3
///     A3 <-- | 12*    *29 | <-> D4
///     A4 <-- | 13*    *28 | <-> D5
///     A5 <-- | 14*    *27 | <-> D6
///     A6 <-- | 15*    *26 | <-> D7
///     A7 <-- | 16*    *25 | --> A15
///     A8 <-- | 17*    *24 | --> A14
///     A9 <-- | 18*    *23 | --> A13
///    A10 <-- | 19*    *22 | --> A12
///    A11 <-- | 20*     21 | --> GND
///            --------------
///
///    * - tri-state, @ - async, / - active on low
/// ```
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
}

impl Pins for W65C02_Pins {
    fn pins(&self) -> &[Rc<Pin>] {
        &self.pins
    }
}
