use crate::emulator::abstractions::{IPin, Pin, PinDirection, Tickable};
use gametime::{Frequency, FrequencyTicker, TimeStamp};
use std::rc::Rc;

pub struct Oscilator {
    pub pin: Rc<Pin>,
    ticker: FrequencyTicker,
}

impl Oscilator {
    pub fn new(khz: u64) -> Self {
        Oscilator {
            pin: Pin::output(),
            ticker: Frequency::from_khz(khz).ticker(TimeStamp::start()),
        }
    }
}

impl Tickable for Oscilator {
    fn tick(&self) {
        self.pin.toggle();
    }
}
