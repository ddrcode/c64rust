use crate::emulator::abstractions::{
    Addr, Addressable, IPin, Pin, PinBuilder,
    PinDirection::{self, *},
    PinStateChange, Port,
};
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

const ADDR_PINS: [usize; 15] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 25, 24, 21, 23, 26];
const DATA_PINS: [usize; 8] = [11, 12, 13, 15, 16, 17, 18, 19];

pub struct HM62256BPins {
    pins: [Rc<Pin>; 28],
    data: Port<u8>,
    addr: Port<u16>,
}

impl HM62256BPins {
    pub fn new() -> Rc<Self> {
        let pins = PinBuilder::new(28)
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
            .build();

        let data_pins: Vec<Rc<Pin>> = DATA_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();
        let addr_pins: Vec<Rc<Pin>> = ADDR_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();

        let res = Rc::new(HM62256BPins {
            pins: pins
                .try_into()
                .unwrap_or_else(|_| panic!("Must have 28 pins")),
            data: Port::from_pins(8, data_pins),
            addr: Port::from_pins(15, addr_pins),
        });

        res.pins[19]
            .set_handler(Rc::clone(&res) as Rc<dyn PinStateChange>)
            .unwrap();
        res.pins[26]
            .set_handler(Rc::clone(&res) as Rc<dyn PinStateChange>)
            .unwrap();

        res
    }
}

impl PinStateChange for HM62256BPins {
    fn on_state_change(&self, pin: &dyn IPin) {
        match &*pin.name().unwrap() {
            "CS" => self.pins.iter().filter(|p| p.tri_state()).for_each(|p| {
                p.set_enable(!pin.state()).unwrap();
            }),

            "WE" => self.data.set_direction(PinDirection::from(!pin.state())),

            _ => {}
        };
    }
}

pub struct HM62256BLogic {
    data: [u8; 1 << 15],
}

impl HM62256BLogic {
    pub fn new() -> Self {
        HM62256BLogic { data: [0; 1 << 15] }
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

pub struct HM62256B<T: Addressable> {
    pins: Rc<HM62256BPins>,
    logic: RefCell<T>,
}

impl<T: Addressable + 'static> HM62256B<T> {
    pub fn new(logic: T) -> Rc<Self> {
        let pins = HM62256BPins::new();
        let logic = RefCell::new(logic);
        let res = Rc::new(HM62256B { pins, logic });

        let handler = Rc::clone(&res) as Rc<dyn PinStateChange>;
        res.pins.data.set_handler(&handler).unwrap();
        res.pins.addr.set_handler(&handler).unwrap();

        res
    }
}

impl<T: Addressable> PinStateChange for HM62256B<T> {
    fn on_state_change(&self, pin: &dyn IPin) {
        if pin.name().is_none() {
            return ();
        }
        let name = pin.name().unwrap();
        let addr = self.pins.addr.read();
        if name == "D" {
            let byte = self.pins.data.read();
            self.logic.borrow_mut().write_byte(addr, byte);
        } else if name == "A" && self.pins.pins[26].low() {
            let byte = self.logic.borrow().read_byte(addr);
            self.pins.data.write(byte);
        }
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
            assert!(pin.name().is_some());
        });
    }

    #[test]
    fn test_memory_read() {
        let mut logic = HM62256BLogic::new();
        logic.write_byte(0x200, 0xa0);

        let mem = HM62256B::new(logic);
        let addr: Port<u16> = Port::new(15, Output);
        let data: Port<u8> = Port::new(8, Input);

        Port::link(&addr, &mem.pins.addr).unwrap();
        Port::link(&data, &mem.pins.data).unwrap();

        addr.write(0x100);
        assert_eq!(0, data.read());

        addr.write(0x200);
        assert_eq!(0xa0, data.read());
    }
}
