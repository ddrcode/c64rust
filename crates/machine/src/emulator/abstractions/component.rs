use super::{Pin, PinStateChange};

pub trait Component: PinStateChange {
    fn get_pin(&self, name: &str) -> Option<&Pin>;
}

