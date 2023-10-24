use std::rc::Rc;

use crate::emulator::abstractions::{IPin, Pin, PinStateChange};

pub struct Nand {
    pub in0: Rc<Pin>,
    pub in1: Rc<Pin>,
    pub out: Rc<Pin>,
}

impl Nand {
    pub fn new() -> Rc<Self> {
        let nand = Nand {
            in0: Pin::input(),
            in1: Pin::input(),
            out: Pin::output(),
        };

        let nand_rc = Rc::new(nand);

        let in_0_rc = nand_rc.clone();
        // nand_rc.in0.observe(move |_| { in_0_rc.compute_state() });

        let in_1_rc = nand_rc.clone();
        // nand_rc.in1.observe(move |_| { in_1_rc.compute_state() });

        nand_rc
    }
}

impl PinStateChange for Nand {
    fn on_state_change(&self, _pin: &dyn IPin) {
        let val = !(self.in0.read() && self.in1.read());
        self.out.write(val);
    }
}

