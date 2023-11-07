use std::cell::{OnceCell, RefCell};
use std::convert::From;
use std::{
    ops::{BitOr, BitOrAssign},
    rc::Rc,
};

use num::{
    traits::{PrimInt, Unsigned},
    NumCast,
};

use crate::emulator::EmulatorError;

use super::{Pin, PinDirection, PinStateChange};

pub struct Port<T: Unsigned + Copy> {
    width: T,
    pins: Box<[Rc<Pin>]>,
    handler: OnceCell<Rc<RefCell<dyn PinStateChange>>>,
    self_ref: OnceCell<Rc<Port<T>>>,
}

impl<T> Port<T>
where
    T: Unsigned
        + PrimInt
        + Copy
        + From<<T as BitOr>::Output>
        + Into<usize>
        + BitOrAssign<T>
        + 'static,
{
    pub fn new(name: &str, width: T, direction: PinDirection) -> Rc<Self> {
        let mut v: Vec<Rc<Pin>> = Vec::with_capacity(width.into());
        for _ in 0..width.into() {
            v.push(Rc::new(Pin::new(name, direction, false, false)));
        }
        Port::from_pins(width, v)
    }

    pub fn from_pins(width: T, pins: Vec<Rc<Pin>>) -> Rc<Self> {
        let port = Rc::new(Port {
            width,
            pins: pins.into_boxed_slice(),
            handler: OnceCell::new(),
            self_ref: OnceCell::new(),
        });
        let c = Rc::clone(&port);
        let _ = port.self_ref.set(c);
        port
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn read(&self) -> T {
        let mut s: T = T::zero();
        for i in 0..self.width().into() {
            s |= (<T as NumCast>::from(self.pins[i].val())).unwrap() << i;
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
            let flag: T = (<T as NumCast>::from(1 << i)).unwrap();
            let val = state & flag;
            self.pins[i]
                .write(val > T::zero())
                .expect("Can't write to input pin");
        }
    }

    pub fn set_direction(&self, dir: PinDirection) {
        for i in 0..self.width().into() {
            self.pins[i].set_direction(dir);
        }
    }

    pub fn set_directions(&self, dirs: T) {
        for i in 0..self.width().into() {
            let flag: T = (<T as NumCast>::from(1 << i)).unwrap();
            let val = dirs & flag;
            self.pins[i].set_direction((val > T::zero()).into());
        }
    }

    pub fn set_handler(
        &self,
        handler: Rc<RefCell<dyn PinStateChange>>,
    ) -> Result<(), EmulatorError> {
        // for i in 0..self.width().into() {
        //     let h = Rc::clone(&self.self_ref.get().unwrap());
        //     self.pins[i].set_handler(h)?;
        // }
        self.handler
            .set(handler)
            .map_err(|_| EmulatorError::HandlerAlreadyDefined("port".to_string()))
    }
}

// impl<T: Copy + Unsigned> PinStateChange for Port<T> {
//     fn on_state_change(&mut self, pin: &Pin) {
//         self.handler.get().unwrap().on_state_change(pin);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_port_creation() {
        let p: Rc<Port<u8>> = Port::new("A", 8, PinDirection::Output);
        assert_eq!(0, p.read());

        p.set_directions(0xff);
        assert_eq!(0xff, p.directions());

        p.write(0xff);
        assert_eq!(0xff, p.read());
    }

    #[test]
    fn test_u16_port_creation() {
        let p: Rc<Port<u16>> = Port::new("A", 16, PinDirection::Output);
        assert_eq!(0, p.read());

        p.set_directions(0xff);
        assert_eq!(0xff, p.directions());

        p.write(0xff);
        assert_eq!(0xff, p.read());
    }
}
