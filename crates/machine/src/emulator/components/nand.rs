use std::rc::Rc;

use crate::emulator::abstractions::{Pin, PinStateChange};

pub struct NandPins {
    pub in1: Pin,
    pub in2: Pin,
    pub out: Pin,
}

impl NandPins {
    pub fn new() -> Self {
        NandPins {
            in1: Pin::input("in1"),
            in2: Pin::input("in2"),
            out: Pin::output("out"),
        }
    }
}

pub trait INand {
    fn execute(&self, in0: bool, in1: bool) -> bool {
        !(in0 && in1)
    }
}

pub struct NandImpl;
impl INand for NandImpl {}

pub struct Nand<T: INand> {
    pins: NandPins,
    logic: T,
}

impl<T: INand + 'static> Nand<T> {
    pub fn new(logic: T) -> Self {
        let pins = NandPins::new();
        Nand { pins, logic }
    }
}

impl<T: INand> PinStateChange for Nand<T> {
    fn on_state_change(&mut self, _pin: &Pin) {
        let val = self
            .logic
            .execute(self.pins.in1.read(), self.pins.in2.read());
        self.pins.out.write(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() {
        let n = NandImpl {};
        assert_eq!(false, n.execute(true, true));
        assert_eq!(true, n.execute(true, false));
        assert_eq!(true, n.execute(false, true));
        assert_eq!(true, n.execute(false, false));
    }

    // #[test]
    // fn test_with_pins() {
    //     let n = Nand::new(NandImpl);
    //     let p1 = Pin::output();
    //     let p2 = Pin::output();
    //
    //     Pin::link(&p1, &n.pins.in1).unwrap();
    //     Pin::link(&p2, &n.pins.in2).unwrap();
    //
    //     p1.write(false);
    //     p2.write(false);
    //     assert_eq!(true, n.pins.out.read());
    // }
}
