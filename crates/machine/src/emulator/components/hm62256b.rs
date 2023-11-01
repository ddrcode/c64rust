use crate::emulator::abstractions::{IPin, Pin, PinBuilder, PinDirection::{*, self}, PinStateChange, Port, Addressable, Addr};
use std::rc::Rc;

const ADDR_PINS: [usize; 15] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 25, 24, 21, 23, 26];
const DATA_PINS: [usize; 8] = [11, 12, 13, 15, 16, 17, 18, 19];

pub struct HM62256BPins {
    pins: [Rc<Pin>; 28],
    data: Port<u8>,
    addr: Port<u16>,
}

impl HM62256BPins {
    pub fn new() -> Self {
        let pins = PinBuilder::new(28)
            .with_ids(&ADDR_PINS)
            .name_prefix("A", 0)
            .direction(Input)
            .with_ids(&DATA_PINS)
            .name_prefix("I/O", 0)
            .io()
            .tri_state()
            .set(14, "VSS", Input)
            .set(20, "CS", Output)
            .set(22, "OE", Input)
            .set(27, "WE", Input)
            .set(28, "VCC", Input)
            .build();

        let data_pins: Vec<Rc<Pin>> = DATA_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();
        let addr_pins: Vec<Rc<Pin>> = ADDR_PINS.map(|id| Rc::clone(&pins[id - 1])).to_vec();

        HM62256BPins {
            pins: pins
                .try_into()
                .unwrap_or_else(|_| panic!("Must have 28 pins")),
            data: Port::from_pins(8, data_pins),
            addr: Port::from_pins(14, addr_pins),
        }
    }
}

impl PinStateChange for HM62256BPins {
    fn on_state_change(&self, pin: &dyn IPin) {
        match &*pin.name().unwrap() {
            "CS" => self
                .pins
                .iter()
                .filter(|p| p.tri_state())
                .for_each(|p| { p.set_enable(!pin.state()).unwrap(); }),

            "WE" => self.data.set_direction(PinDirection::from(!pin.state())),

            _ => {}
        };
    }
}

pub struct HM62256BLogic {
    data: [u8; 1<<15]
}

impl HM62256BLogic {
    pub fn new() -> Self {
        HM62256BLogic {
            data: [0; 1<<15]
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

pub struct HM62256B {
    pins: HM62256BPins,
    logic: HM62256BLogic
}

impl HM62256B {
    pub fn new() -> Self {
        let pins = HM62256BPins::new();
        let logic = HM62256BLogic::new();
        HM62256B { pins, logic }
    }
}

impl PinStateChange for HM62256B {
    fn on_state_change(&self, pin: &dyn IPin) {
        todo!()
    }
}

