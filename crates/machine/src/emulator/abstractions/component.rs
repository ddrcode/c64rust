use std::any::Any;

use super::{Pin, PinStateChange};

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait Component: PinStateChange + AsAny {
    fn get_pin(&self, name: &str) -> Option<&Pin>;
}
