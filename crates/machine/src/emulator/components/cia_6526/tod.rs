use crate::utils::{bcd_to_dec, dec_to_bcd, if_else};
use chrono::{Duration, NaiveTime, Timelike};
use std::cell::RefCell;

pub struct TOD {
    read_lock: RefCell<bool>,
    write_lock: RefCell<bool>,
    locked_val: RefCell<NaiveTime>,
    hour_offset: Duration,
    minute_offset: Duration,
    second_offset: Duration,
}

impl TOD {
    pub fn new() -> Self {
        TOD {
            read_lock: RefCell::new(false),
            write_lock: RefCell::new(false),
            locked_val: RefCell::new(TOD::now()),
            hour_offset: Duration::hours(0),
            minute_offset: Duration::minutes(0),
            second_offset: Duration::seconds(0),
        }
    }

    fn now() -> NaiveTime {
        chrono::Local::now().time()
    }

    fn local_now(&self) -> NaiveTime {
        TOD::now() + self.hour_offset + self.minute_offset + self.second_offset
    }

    fn is_locked(&self) -> bool {
        *self.read_lock.borrow() || *self.write_lock.borrow()
    }

    fn lock_read(&self) {
        if !self.is_locked() {
            *self.read_lock.borrow_mut() = true;
            *self.locked_val.borrow_mut() = self.local_now();
        }
    }

    fn lock_write(&self) {
        self.lock_read();
        *self.write_lock.borrow_mut() = true;
    }

    fn unloack_read(&self) {
        *self.read_lock.borrow_mut() = false;
    }

    fn unlock_write(&self) {
        *self.write_lock.borrow_mut() = false;
    }

    fn time(&self) -> NaiveTime {
        if self.is_locked() {
            *self.locked_val.borrow()
        } else {
            self.local_now()
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

    pub fn set_hour(&mut self, hour: u8) {
        let hour_dec = bcd_to_dec(hour & 0b01111111) + 12 * (hour >> 7);
        self.hour_offset = Duration::hours(hour_dec as i64 - TOD::now().hour() as i64);
        self.lock_write();
    }

    pub fn set_minute(&mut self, minute: u8) {
        self.minute_offset =
            Duration::minutes(bcd_to_dec(minute) as i64 - TOD::now().minute() as i64);
    }

    pub fn set_second(&mut self, second: u8) {
        self.second_offset =
            Duration::seconds(bcd_to_dec(second) as i64 - TOD::now().second() as i64);
    }

    pub fn set_tenth(&mut self, _val: u8) {
        self.unlock_write();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_hour() {
        let mut tod = TOD::new();
        tod.set_hour(5);
        tod.set_tenth(0);
        assert_eq!(5, tod.hour());
    }
}
