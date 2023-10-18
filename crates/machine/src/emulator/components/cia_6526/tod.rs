use std::cell::RefCell;
use crate::utils::{dec_to_bcd, if_else};
use chrono::{NaiveTime, Timelike};

pub struct TOD {
    read_lock: RefCell<bool>,
    write_lock: RefCell<bool>,
    locked_val: RefCell<NaiveTime>,
}

impl TOD {
    pub fn new() -> Self {
        TOD {
            read_lock: RefCell::new(false),
            write_lock: RefCell::new(false),
            locked_val: RefCell::new(TOD::now()),
        }
    }

    fn now() -> NaiveTime {
        chrono::Local::now().time()
    }

    fn is_locked(&self) -> bool {
        *self.read_lock.borrow() || *self.write_lock.borrow()
    }

    fn lock_read(&self) {
        if !self.is_locked() {
            *self.read_lock.borrow_mut() = true;
            *self.locked_val.borrow_mut() = TOD::now();
        }
    }

    fn unloack_read(&self) {
        *self.read_lock.borrow_mut() = false;
    }

    fn time(&self) -> NaiveTime {
        if self.is_locked() {
            *self.locked_val.borrow()
        } else {
            TOD::now()
        }
    }

    pub fn hour(&self) -> u8 {
        self.lock_read();
        let (pm, hour) = self.time().hour12();
        let pm_flag = if_else(pm, 128, 0);
        dec_to_bcd(hour as u8) | pm_flag
    }

    pub fn minute(&self) -> u8 {
        dec_to_bcd(self.time().minute() as u8)
    }

    pub fn second(&self) -> u8 {
        dec_to_bcd(self.time().second() as u8)
    }

    pub fn tenth(&self) -> u8 {
        let t = (self.time().nanosecond() / 100_000_000) % 10;
        self.unloack_read();
        dec_to_bcd(t as u8)
    }
}
