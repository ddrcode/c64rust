use std::{ops::{BitOrAssign, BitOr}, rc::Rc};
use std::convert::From;

use num::{traits::{PrimInt, Unsigned}, NumCast};

use crate::emulator::EmulatorError;

use super::{IPin, Pin, PinDirection, PinStateChange};

pub struct Port<T: Unsigned + Copy> {
    width: T,
    pins: Box<[Rc<Pin>]>,
}

impl<T: Unsigned + PrimInt + Copy + From<<T as BitOr>::Output> + Into<usize> + BitOrAssign<T>> Port<T> {
    pub fn new(width: T, direction: PinDirection) -> Self {
        let mut v: Vec<Rc<Pin>> = Vec::with_capacity(width.into());
        for _ in 0..width.into() {
            v.push(Pin::new(direction, false, false));
        }
        Port::from_pins(width, v)
    }

    pub fn from_pins(width: T, pins: Vec<Rc<Pin>>) -> Self {
        Port {
            width,
            pins: pins.into_boxed_slice()
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

    pub fn read(&self) -> T {
        let mut s: T = T::zero();
        for i in 0..self.width().into() {
            s |= (<T as NumCast>::from(self.pins[i].val())).unwrap()  << i;
        }
        s
    }

    pub fn directions(&self) -> T {
        let mut s: T = T::zero();
        for i in 0..self.width().into() {
            s |= (<T as NumCast>::from(self.pins[i].direction() as u8)).unwrap() << i;
        }
        s
    }

    pub fn write(&self, state: T) {
        for i in 0..self.width().into() {
            let flag: T = (<T as  NumCast>::from(1 << i)).unwrap();
            let val = state & flag;
            self.pins[i].write(val > T::zero());
        }
    }

    pub fn set_direction(&self, dir: PinDirection) {
        for i in 0..self.width().into() {
            self.pins[i].set_direction(dir);
        }
    }

    pub fn set_directions(&self, dirs: T) {
        for i in 0..self.width().into() {
            let flag: T = (<T as  NumCast>::from(1 << i)).unwrap();
            let val = dirs & flag;
            self.pins[i].set_direction((val > T::zero()).into());
        }
    }

    pub fn set_handler(&self, handler: &Rc<dyn PinStateChange>) -> Result<(), EmulatorError> {
        for i in 0..self.width().into() {
            self.pins[i].set_handler(Rc::clone(handler))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_port_creation() {
        let p: Port<u8> = Port::new(8, PinDirection::Input);
        assert_eq!(0, p.read());

        p.set_directions(0xff);
        assert_eq!(0xff, p.directions());

        p.write(0xff);
        assert_eq!(0xff, p.read());
    }

    #[test]
    fn test_u16_port_creation() {
        let p: Port<u16> = Port::new(16, PinDirection::Input);
        assert_eq!(0, p.read());

        p.set_directions(0xff);
        assert_eq!(0xff, p.directions());

        p.write(0xff);
        assert_eq!(0xff, p.read());
    }
}
