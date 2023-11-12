use crate::emulator::abstractions::{
    Addr, Addressable, Component, Pin, PinBuilder,
    PinDirection::{self, *},
    PinStateChange, Pins, Port, RAM, AsAny,
};
use std::collections::HashMap;
use std::rc::Rc;

const ADDR_PINS: [usize; 15] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 25, 24, 21, 23, 26];
const DATA_PINS: [usize; 8] = [11, 12, 13, 15, 16, 17, 18, 19];

/// ```text
///               HM62256B
///            ----------------
///    A14 --> |  1 *      28 | <-- VCC
///    A12 --> |  2 *      27 | <-- WE!
///     A7 --> |  3 *    * 26 | <-- A13
///     A6 --> |  4 *    * 25 | <-- A8
///     A5 --> |  5 *    * 24 | <-- A9
///     A4 --> |  6 *    * 23 | <-- A11
///     A3 --> |  7 *      22 | --- OE!
///     A2 --> |  8 *    * 21 | <-- A10
///     A1 --> |  9 *      20 | <-- CS!
///     A0 --> | 10 *    * 19 | <-> IO7
///    IO0 <-> | 11 *    * 18 | <-> IO6
///    IO1 <-> | 12 *    * 17 | <-> IO5
///    IO2 <-> | 13 *    * 16 | <-> IO4
///    GND <-- | 14      * 15 | <-> IO3
///            ----------------
///
///    * - tri-state, @ - async, ! - active on low
/// ```
pub struct HM62256BPins {
    pins: [Rc<Pin>; 28],
    pins_map: HashMap<String, Rc<Pin>>,
    pub data: Rc<Port<u8>>,
    pub addr: Rc<Port<u16>>,
}

impl HM62256BPins {
    pub fn new() -> Self {
        let pins: Vec<Rc<Pin>> = PinBuilder::new(28)
            .with_ids(&ADDR_PINS)
            .group("A", 0)
            .direction(Input)
            .with_ids(&DATA_PINS)
            .group("D", 0)
            .direction(Output)
            .io()
            .tri_state()
            .set(14, "VSS", Input)
            .set(20, "CS", Input)
            .set(22, "OE", Input)
            .set(27, "WE", Input)
            .set(28, "VCC", Input)
            .build()
            .iter()
            .map(|pin| Rc::new(pin.clone()))
            .collect();

        let data_pins: Vec<Rc<Pin>> = DATA_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();
        let addr_pins: Vec<Rc<Pin>> = ADDR_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();

        let mut pins_map: HashMap<String, Rc<Pin>> = HashMap::with_capacity(40);
        pins.iter().for_each(|pin| {
            pins_map.insert(pin.name().to_string(), Rc::clone(pin));
        });

        HM62256BPins {
            pins: pins
                .try_into()
                .unwrap_or_else(|_| panic!("Must have 28 pins")),
            data: Port::from_pins(8, data_pins),
            addr: Port::from_pins(15, addr_pins),
            pins_map,
        }
    }
}

impl Pins for HM62256BPins {
    fn pins(&self) -> &[Rc<Pin>] {
        &self.pins
    }
}

pub struct HM62256BLogic {
    data: [u8; 1 << 15],
}

impl HM62256BLogic {
    pub fn new() -> Self {
        HM62256BLogic { data: [0; 1 << 15] }
    }

    pub fn load(&mut self, addr: Addr, data: &[u8]) {
        let a = addr as usize;
        for i in a..a + data.len() {
            self.data[i] = data[i - a];
        }
    }
}

impl Addressable for HM62256BLogic {
    fn read_byte(&self, addr: Addr) -> u8 {
        self.data[addr as usize]
    }

    fn write_byte(&mut self, addr: Addr, value: u8) {
        self.data[addr as usize] = value;
    }

    fn address_width(&self) -> u16 {
        15
    }
}

impl RAM for HM62256BLogic {}

pub struct HM62256B<T: Addressable> {
    pub pins: Rc<HM62256BPins>,
    pub logic: T,
}

impl<T: RAM > HM62256B<T> {
    pub fn new(logic: T) -> Self {
        let pins = Rc::new(HM62256BPins::new());
        HM62256B { pins, logic }
    }
}

impl<T: RAM+'static> Component for HM62256B<T> {
    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.by_name(name)
    }
}

impl<T: RAM> PinStateChange for HM62256B<T> {
    fn on_state_change(&mut self, pin: &Pin) {
        let addr = self.pins.addr.read();
        match &*pin.name() {
            "D" => {
                let byte = self.pins.data.read();
                self.logic.write_byte(addr, byte);
            }
            "A" => {
                if self.pins.pins[26].low() {
                    let byte = self.logic.read_byte(addr);
                    self.pins.data.write(byte);
                }
            }
            "CS" => self
                .pins
                .pins
                .iter()
                .filter(|p| p.tri_state())
                .for_each(|p| {
                    p.set_enable(!pin.state()).unwrap();
                }),
            "WE" => self
                .pins
                .data
                .set_direction(PinDirection::from(!pin.state()))
                .unwrap(),
            _ => {}
        }
    }
}

impl<T: RAM + 'static> AsAny for HM62256B<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let mem = HM62256B::new(HM62256BLogic::new());
        mem.pins.pins.iter().for_each(|pin| {
            assert!(pin.id().is_some());
        });
    }

    // #[test]
    // fn test_memory_read() {
    //     let mut logic = HM62256BLogic::new();
    //     logic.write_byte(0x200, 0xa0);
    //
    //     let mem = HM62256B::new(logic);
    //     let addr: Port<u16> = Port::new(15, Output);
    //     let data: Port<u8> = Port::new(8, Input);
    //
    //     Port::link(&addr, &mem.pins.addr).unwrap();
    //     Port::link(&data, &mem.pins.data).unwrap();
    //
    //     addr.write(0x100);
    //     assert_eq!(0, data.read());
    //
    //     addr.write(0x200);
    //     assert_eq!(0xa0, data.read());
    // }
    //
    // #[test]
    // fn test_memory_write() {
    //     let logic = HM62256BLogic::new();
    //     let mem = HM62256B::new(logic);
    //     let addr: Rc<Port<u16>> = Port::new(15, Output);
    //     let data: Rc<Port<u8>> = Port::new(8, Output);
    //     let we = Pin::output();
    //
    //     Pin::link(&we, &mem.pins.pins[26]).unwrap();
    //     Port::link(&addr, &mem.pins.addr).unwrap();
    //     Port::link(&data, &mem.pins.data).unwrap();
    //
    //     we.set_low();
    //     assert_eq!(false, mem.pins.pins[26].read());
    //
    //     addr.write(0x100);
    //     data.write(128);
    //
    //     assert_eq!(128, mem.logic.borrow().read_byte(0x100));
    // }
}
