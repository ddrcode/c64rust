use crate::{
    emulator::abstractions::{Component, Pin, PinStateChange, Tickable},
    utils::if_else,
};
use gametime::{Frequency, FrequencyTicker, TimeStamp};

pub struct Oscilator {
    pub pin: Pin,
    ticker: FrequencyTicker,
}

impl Oscilator {
    pub fn new(khz: u64) -> Self {
        Oscilator {
            pin: Pin::output("OUT"),
            ticker: Frequency::from_khz(khz).ticker(TimeStamp::start()),
        }
    }
}

impl Tickable for Oscilator {
    fn tick(&self) {
        self.pin.toggle().unwrap();
    }
}

impl Component for Oscilator {
    fn get_pin(&self, name: &str) -> Option<&Pin> {
        if_else(name == "OUT", Some(&self.pin), None)
    }
}

impl PinStateChange for Oscilator {
    fn on_state_change(&mut self, _pin: &Pin) {
        // no input pins
    }
}
