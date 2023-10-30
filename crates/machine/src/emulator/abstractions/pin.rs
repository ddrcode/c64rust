use std::{
    cell::{OnceCell, RefCell},
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
    name: OnceCell<String>,
    value: RefCell<bool>,
    enabled: RefCell<bool>,
    direction: RefCell<PinDirection>,
    connection: RefCell<Weak<Pin>>,
    handler: OnceCell<Rc<dyn PinStateChange>>,
    tri_state: bool,
    io: bool,
}

pub trait IPin {
    fn state(&self) -> bool;

    fn read(&self) -> bool {
        self.state()
    }

    fn val(&self) -> u8 {
        self.state().into()
    }

    fn high(&self) -> bool {
        self.state()
    }

    fn low(&self) -> bool {
        !self.state()
    }

    fn direction(&self) -> PinDirection;

    fn linked(&self) -> bool;

    fn name(&self) -> Option<String>;

    fn is_output(&self) -> bool {
        self.direction() == PinDirection::Output
    }

    fn write(&self, val: bool);

    fn set_direction(&self, dir: PinDirection);

    fn set_high(&self) {
        self.write(true);
    }

    fn set_low(&self) {
        self.write(false);
    }

    fn toggle(&self) {
        if self.is_output() {
            let v = self.state();
            self.write(!v);
        }
    }

    fn tri_state(&self) -> bool;

    fn enabled(&self) -> bool {
        true
    }

    fn enable(&self) -> Result<(), EmulatorError>;
    fn disable(&self) -> Result<(), EmulatorError>;
}

impl Into<u8> for Pin {
    fn into(self) -> u8 {
        self.state().into()
    }
}

impl Pin {
    pub fn new(direction: PinDirection, tri_state: bool, io: bool) -> Rc<Self> {
        let pin = Pin {
            name: OnceCell::new(),
            value: RefCell::new(false),
            enabled: RefCell::new(true),
            direction: RefCell::new(direction),
            connection: RefCell::new(Weak::new()),
            handler: OnceCell::new(),
            tri_state,
            io,
        };
        Rc::new(pin)
    }

    pub fn input() -> Rc<Self> {
        Pin::new(PinDirection::Input, false, false)
    }

    pub fn output() -> Rc<Self> {
        Pin::new(PinDirection::Output, false, false)
    }

    pub fn link(pin1: &Rc<Pin>, pin2: &Rc<Pin>) -> Result<(), EmulatorError> {
        if pin1.linked() || pin2.linked() {
            return Err(EmulatorError::PinAlreadyLinked);
        }
        *pin1.connection.borrow_mut() = Rc::downgrade(pin2);
        *pin2.connection.borrow_mut() = Rc::downgrade(pin1);
        Ok(())
    }

    pub fn set_name(&self, name: String) {
        self.name.set(name);
    }

    pub fn set_handler(&self, handler: Rc<dyn PinStateChange>) -> Result<(), EmulatorError> {
        self.handler
            .set(handler)
            .map_err(|_| EmulatorError::HandlerAlreadyDefined)
    }

    fn set_enable(&self, val: bool) -> Result<(), EmulatorError> {
        if !self.tri_state {
            return Err(EmulatorError::NotATriStatePin);
        }
        *self.enabled.borrow_mut() = val;
        Ok(())
    }
}

impl IPin for Pin {
    fn state(&self) -> bool {
        let linked = (*self.connection.borrow()).upgrade();
        if linked.clone().map_or(false, |port| port.is_output()) {
            *linked.unwrap().value.borrow()
        } else if self.is_output() {
            *self.value.borrow()
        } else {
            false
        }
    }

    fn direction(&self) -> PinDirection {
        *self.direction.borrow()
    }

    fn linked(&self) -> bool {
        (*self.connection.borrow()).upgrade().is_some()
    }

    fn write(&self, val: bool) {
        if self.is_output() {
            *self.value.borrow_mut() = val;
            if let Some(pin) = (*self.connection.borrow()).upgrade() {
                if let Some(handler) = pin.handler.get() {
                    handler.on_state_change(self);
                }
            }
        }
    }

    fn set_direction(&self, dir: PinDirection) {
        *self.direction.borrow_mut() = dir;
    }

    fn name(&self) -> Option<String> {
        self.name.get().cloned()
    }

    fn tri_state(&self) -> bool {
        self.tri_state
    }

    fn enable(&self) -> Result<(), EmulatorError> {
        self.set_enable(true)
    }

    fn disable(&self) -> Result<(), EmulatorError> {
        self.set_enable(false)
    }
}

pub trait PinStateChange {
    fn on_state_change(&self, pin: &dyn IPin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_link() {
        struct A {
            d0: Rc<Pin>,
        }

        let a = A { d0: Pin::input() };
        let b = A { d0: Pin::output() };
        let res = Pin::link(&a.d0, &b.d0);
        assert!(res.is_ok());
        assert!(a.d0.low());
        b.d0.set_high();
        assert!(a.d0.high());
    }

    #[test]
    fn test_direction_change() {
        let pin = Pin::input();
        assert_eq!(PinDirection::Input, pin.direction());
        assert_eq!(0u8, pin.direction().into());
        assert_eq!(false, pin.direction().into());

        pin.set_direction(PinDirection::Output);
        assert_eq!(PinDirection::Output, pin.direction());
        assert_eq!(1u8, pin.direction().into());
        assert_eq!(true, pin.direction().into());
    }
}
