use std::rc::Rc;
use gametime::{ TimeStamp, FrequencyTicker, Frequency };
use crate::emulator::abstractions::{PinDirection, Pin, Tickable};

pub struct Oscilator {
    pub pin: Rc<Pin>,
    ticker: FrequencyTicker,
}

impl Oscilator {
    pub fn new(khz: u64) -> Self {
        Oscilator {
            pin: Pin::new(PinDirection::Output),
            ticker: Frequency::from_khz(khz).ticker(TimeStamp::start()),
        }
    }
}

impl Tickable for Oscilator {
    fn tick(&self) {
        self.pin.toggle();
    }
}
