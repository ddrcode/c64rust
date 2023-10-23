use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::emulator::EmulatorError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PinDirection {
    Input,
    Output,
}

impl Default for PinDirection {
    fn default() -> Self {
        PinDirection::Input
    }
}

impl From<bool> for PinDirection {
    fn from(value: bool) -> Self {
        if value {
            PinDirection::Output
        } else {
            PinDirection::Input
        }
    }
}

impl Into<bool> for PinDirection {
    fn into(self) -> bool {
        self == PinDirection::Output
    }
}

impl Into<u8> for PinDirection {
    fn into(self) -> u8 {
        if self == PinDirection::Output {
            1
        } else {
            0
        }
    }
}

pub struct Pin {
    value: RefCell<bool>,
    direction: RefCell<PinDirection>,
    connection: RefCell<Weak<Pin>>,
    observer: RefCell<Box<dyn Fn(bool)>>
}

impl Pin {
    pub fn new(direction: PinDirection) -> Rc<Self> {
        let pin = Pin {
            value: RefCell::new(false),
            direction: RefCell::new(direction),
            connection: RefCell::new(Weak::new()),
            observer: RefCell::new(Box::new(|_|{}))
        };
        Rc::new(pin)
    }

    pub fn state(&self) -> bool {
        let linked = (*self.connection.borrow()).upgrade();
        if linked.clone().map_or(false, |port| port.output()) {
            *linked.unwrap().value.borrow()
        } else if self.output() {
            *self.value.borrow()
        } else {
            false
        }
    }

    pub fn val(&self) -> u8 {
        self.state().into()
    }

    pub fn high(&self) -> bool {
        self.state()
    }

    pub fn low(&self) -> bool {
        !self.state()
    }

    pub fn direction(&self) -> PinDirection {
        *self.direction.borrow()
    }

    pub fn linked(&self) -> bool {
        (*self.connection.borrow()).upgrade().is_some()
    }

    pub fn link(pin1: &Rc<Pin>, pin2: &Rc<Pin>) -> Result<(), EmulatorError> {
        if pin1.linked() || pin2.linked() {
            return Err(EmulatorError::PinAlreadyLinked);
        }
        *pin1.connection.borrow_mut() = Rc::downgrade(pin2);
        *pin2.connection.borrow_mut() = Rc::downgrade(pin1);
        Ok(())
    }

    pub fn output(&self) -> bool {
        *self.direction.borrow() == PinDirection::Output
    }

    pub fn write(&self, val: bool) {
        if self.output() {
            *self.value.borrow_mut() = val;
            if let Some(pin) = (*self.connection.borrow()).upgrade() {
                (*pin.observer.borrow())(val);
            }
        }
    }

    pub fn set_direction(&self, dir: PinDirection) {
        *self.direction.borrow_mut() = dir;
    }

    pub fn set_high(&self) {
        self.write(true);
    }

    pub fn set_low(&self) {
        self.write(false);
    }

    pub fn toggle(&self) {
        if self.output() {
            let v = *self.value.borrow();
            self.write(!v);
        }
    }

    pub fn observe(&self, observer: impl Fn(bool) + 'static) {
        *self.observer.borrow_mut() = Box::new(observer);
    }
}

impl Into<u8> for Pin {
    fn into(self) -> u8 {
        self.state().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_link() {
        struct A {
            d0: Rc<Pin>,
        }

        let a = A {
            d0: Pin::new(PinDirection::Input),
        };
        let b = A {
            d0: Pin::new(PinDirection::Output),
        };
        let res = Pin::link(&a.d0, &b.d0);
        assert!(res.is_ok());
        assert!(a.d0.low());
        b.d0.set_high();
        assert!(a.d0.high());
    }

    #[test]
    fn test_direction_change() {
        let pin = Pin::new(PinDirection::Input);
        assert_eq!(PinDirection::Input, pin.direction());
        assert_eq!(0u8, pin.direction().into());
        assert_eq!(false, pin.direction().into());

        pin.set_direction(PinDirection::Output);
        assert_eq!(PinDirection::Output, pin.direction());
        assert_eq!(1u8, pin.direction().into());
        assert_eq!(true, pin.direction().into());
    }
}
