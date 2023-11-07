use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
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

#[derive(Clone)]
pub struct Pin {
    id: OnceCell<u8>,
    inner_id: OnceCell<u32>,
    name: String,
    group_id: OnceCell<u8>,
    value: RefCell<bool>,
    enabled: RefCell<bool>,
    direction: RefCell<PinDirection>,
    handler: OnceCell<Rc<RefCell<dyn PinStateChange>>>,
    tri_state: bool,
    io: bool,
}

impl Into<u8> for Pin {
    fn into(self) -> u8 {
        self.state().into()
    }
}

impl Pin {
    pub fn new(name: &str, direction: PinDirection, tri_state: bool, io: bool) -> Self {
        Pin {
            id: OnceCell::new(),
            inner_id: OnceCell::new(),
            name: name.to_string(),
            group_id: OnceCell::new(),
            value: RefCell::new(false),
            enabled: RefCell::new(true),
            direction: RefCell::new(direction),
            handler: OnceCell::new(),
            tri_state,
            io,
        }
    }

    pub fn input(name: &str) -> Self {
        Pin::new(name, PinDirection::Input, false, false)
    }

    pub fn output(name: &str) -> Self {
        Pin::new(name, PinDirection::Output, false, false)
    }

    pub fn set_id(&self, id: u8) {
        let _ = self.id.set(id);
    }

    pub fn set_group_id(&self, id: u8) {
        let _ = self.group_id.set(id);
    }

    pub(crate) fn set_handler(
        &self,
        handler: Rc<RefCell<dyn PinStateChange>>,
    ) -> Result<(), EmulatorError> {
        self.handler
            .set(handler)
            .map_err(|_| EmulatorError::HandlerAlreadyDefined(self.name()))
    }

    pub fn set_enable(&self, val: bool) -> Result<(), EmulatorError> {
        if !self.tri_state {
            return Err(EmulatorError::NotATriStatePin);
        }
        *self.enabled.borrow_mut() = val;
        Ok(())
    }

    pub fn read(&self) -> bool {
        self.state()
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

    pub fn name(&self) -> String {
        let name = &self.name;
        let group_id = self.group_id();
        if group_id.is_some() {
            format!("{}{}", name, group_id.unwrap())
        } else {
            name.to_string()
        }
    }

    pub fn group_name(&self) -> Option<String> {
        let name = self.name();
        let group_id = self.group_id();
        if group_id.is_some() {
            Some(name)
        } else {
            None
        }
    }

    pub fn is_output(&self) -> bool {
        self.direction() == PinDirection::Output
    }

    pub fn set_high(&self) -> Result<bool, EmulatorError> {
        self.write(true)
    }

    pub fn set_low(&self) -> Result<bool, EmulatorError> {
        self.write(false)
    }

    pub fn toggle(&self) -> Result<bool, EmulatorError> {
        let v = self.state();
        self.write(!v)
    }

    pub fn enabled(&self) -> bool {
        true
    }

    pub fn state(&self) -> bool {
        *self.value.borrow()
    }

    pub fn direction(&self) -> PinDirection {
        *self.direction.borrow()
    }

    pub fn write(&self, val: bool) -> Result<bool, EmulatorError> {
        if self.is_output() {
            if *self.value.borrow() == val {
                return Ok(false);
            }
            *self.value.borrow_mut() = val;
            if let Some(handler) = self.handler.get() {
                handler.borrow_mut().on_state_change(self);
            }
            Ok(true)
        } else {
            Err(EmulatorError::CantWriteToReadPin(self.name()))
        }
    }

    pub(crate) fn set_val(&self, val: bool) {
        if !self.is_output() {
            *self.value.borrow_mut() = val;
        }
    }

    pub fn set_direction(&self, dir: PinDirection) {
        *self.direction.borrow_mut() = dir;
    }

    pub fn id(&self) -> Option<u8> {
        self.id.get().copied()
    }

    pub fn group_id(&self) -> Option<u8> {
        self.group_id.get().copied()
    }

    pub fn tri_state(&self) -> bool {
        self.tri_state
    }

    pub fn enable(&self) -> Result<(), EmulatorError> {
        self.set_enable(true)
    }

    pub fn disable(&self) -> Result<(), EmulatorError> {
        self.set_enable(false)
    }

    pub fn inner_id(&self) -> Option<u32> {
        self.inner_id.get().copied()
    }

    pub(crate) fn set_inner_id(&self, id: u32) {
        let _ = self.inner_id.set(id);
    }
}

pub trait PinStateChange {
    fn on_state_change(&mut self, pin: &Pin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_change() {
        let pin = Pin::input("a");
        assert_eq!(PinDirection::Input, pin.direction());
        assert_eq!(0u8, pin.direction().into());
        assert_eq!(false, pin.direction().into());

        pin.set_direction(PinDirection::Output);
        assert_eq!(PinDirection::Output, pin.direction());
        assert_eq!(1u8, pin.direction().into());
        assert_eq!(true, pin.direction().into());
    }
}
