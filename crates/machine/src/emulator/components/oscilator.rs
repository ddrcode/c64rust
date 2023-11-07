use crate::emulator::abstractions::{Pin, Tickable};
use gametime::{Frequency, FrequencyTicker, TimeStamp};

pub struct Oscilator {
    pub pin: Pin,
    ticker: FrequencyTicker,
}

impl Oscilator {
    pub fn new(khz: u64) -> Self {
        Oscilator {
            pin: Pin::output("phi1o"),
            ticker: Frequency::from_khz(khz).ticker(TimeStamp::start()),
        }
    }
}

impl Tickable for Oscilator {
    fn tick(&self) {
        self.pin.toggle();
    }
}
