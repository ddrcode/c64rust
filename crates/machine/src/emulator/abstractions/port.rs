use std::{ops::BitOrAssign, rc::Rc};

use num::traits::{PrimInt, Unsigned};

use crate::emulator::EmulatorError;

use super::{IPin, Pin, PinDirection};

pub struct Port<T: Unsigned + Copy> {
    width: T,
    pins: Box<[Rc<Pin>]>,
}

impl<T: Unsigned + PrimInt + Copy + Into<usize> + From<u8> + BitOrAssign<T>> Port<T> {
    pub fn new(width: T, direction: PinDirection) -> Self {
        let mut v: Vec<Rc<Pin>> = Vec::with_capacity(width.into());
        for _ in 0..width.into() {
            v.push(Pin::new(direction));
        }
        Port {
            width,
            pins: v.into_boxed_slice(),
        }
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn link(port_a: &Port<T>, port_b: &Port<T>) -> Result<(), EmulatorError> {
        if port_a.width() != port_b.width() {
            return Err(EmulatorError::IncompatiblePortWidths);
        }
        for i in 0..port_a.width().into() {
            Pin::link(&port_a.pins[i], &port_b.pins[i])?;
        }
        Ok(())
    }

    pub fn state(&self) -> T {
        let mut s: T = (0u8).into();
        for i in 0..self.width().into() {
            s |= <T as From<u8>>::from(self.pins[i].val()) << i;
        }
        s
    }

    pub fn directions(&self) -> T {
        let mut s: T = (0u8).into();
        for i in 0..self.width().into() {
            s |= <T as From<u8>>::from(self.pins[i].direction().into()) << i;
        }
        s
    }

    pub fn set_state(&self, state: T) {
        for i in 0..self.width().into() {
            let flag: T = (1 << i).into();
            let val = state & flag;
            self.pins[i].write(val > 0.into());
        }
    }

    pub fn set_directions(&self, dirs: T) {
        for i in 0..self.width().into() {
            let flag: T = (1 << i).into();
            let val = dirs & flag;
            self.pins[i].set_direction((val > 0.into()).into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_creation() {
        let p: Port<u8> = Port::new(8, PinDirection::Input);
        assert_eq!(0, p.state());

        p.set_directions(0xff);
        assert_eq!(0xff, p.directions());

        p.set_state(0xff);
        assert_eq!(0xff, p.state());
    }
}
