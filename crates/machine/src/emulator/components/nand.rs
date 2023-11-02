use std::rc::Rc;

use crate::emulator::abstractions::{IPin, Pin, PinStateChange};

pub struct NandPins {
    pub in0: Rc<Pin>,
    pub in1: Rc<Pin>,
    pub out: Rc<Pin>,
}

impl NandPins {
    pub fn new() -> Self {
        NandPins {
            in0: Pin::input(),
            in1: Pin::input(),
            out: Pin::output(),
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
    pub fn new(logic: T) -> Rc<Self> {
        let pins = NandPins::new();
        let nand = Nand { pins, logic };

        let nand_rc = Rc::new(nand);

        let c0 = Rc::clone(&nand_rc);
        nand_rc.pins.in0.set_handler(c0).unwrap();

        let c1 = Rc::clone(&nand_rc);
        nand_rc.pins.in1.set_handler(c1).unwrap();

        nand_rc
    }
}

impl<T: INand> PinStateChange for Nand<T> {
    fn on_state_change(&self, _pin: &dyn IPin) {
        let val = self
            .logic
            .execute(self.pins.in0.read(), self.pins.in1.read());
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

    #[test]
    fn test_with_pins() {
        let n = Nand::new(NandImpl);
        let p1 = Pin::output();
        let p2 = Pin::output();

        Pin::link(&p1, &n.pins.in0).unwrap();
        Pin::link(&p2, &n.pins.in1).unwrap();

        p1.write(false);
        p2.write(false);
        assert_eq!(true, n.pins.out.read());
    }
}
