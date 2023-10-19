use crate::utils::{bcd_to_dec, dec_to_bcd, if_else};
use chrono::{Duration, NaiveTime, Timelike};
use std::cell::RefCell;
use std::time::Instant;

/// The structure holding internal state of Time-of-day clock of CIA6526.
/// The CIA clock provides (and accepts) values in BCD format.
/// Reading/setting the hour freezes the clock, until the corresponding
/// reading/setting the tenths of second. Reading freezes the clock virtually
/// (doesn't stop the actuall clock ticking), while writing halts the clock
/// completely.
/// The most accurate documenation is provided by Wikipedia
/// [MOS Technology CIA](https://en.wikipedia.org/wiki/MOS_Technology_CIA)
pub struct TOD {
    /// The flag is set to true on call of hour() method.
    /// If set, every read of time returns "frozen" value (from locked_val variable)
    /// rather than actual time. Calling tenth() method unfreezes clock reads.
    /// Unlike write_lock, read_lock doesn't halt the clock completely, so the time flow
    /// continues.
    read_lock: RefCell<bool>,
    locked_val: RefCell<NaiveTime>,

    /// Indicates TOD being halted (no clock "ticking"). It happens
    /// automatically on calling set_hour. Clock ticking continues
    /// on set_tenth
    write_lock: bool,

    /// Hours offset between host's local time and TOD time (0 by default)
    hour_offset: Duration,

    /// Minutes offset between host's local time and TOD time (0 by default)
    minute_offset: Duration,

    /// Seconds offset between host's local time and TOD time (0 by default)
    second_offset: Duration,

    /// Millis offset between host's local time and TOD time (0 by default)
    millis_offset: Duration,

    /// cumulative time (in ms) between set_hour (when CIA clock is halted)
    /// and set_tenth (clock unhalted)
    halt_offset: Duration,

    /// Used for measuring duration of a single clock halt
    last_halt_duration: Option<Instant>,
}

impl TOD {
    /// Creates new time-of-day clock. By default the clock is in halted mode
    /// so to make it ticking set_tenth must be called first
    pub fn new() -> Self {
        TOD {
            read_lock: RefCell::new(false),
            write_lock: true,
            locked_val: RefCell::new(TOD::now()),
            hour_offset: Duration::hours(0),
            minute_offset: Duration::minutes(0),
            second_offset: Duration::seconds(0),
            millis_offset: Duration::milliseconds(0),
            halt_offset: Duration::seconds(0),
            last_halt_duration: Some(Instant::now()),
        }
    }

    fn now() -> NaiveTime {
        chrono::Local::now().time()
    }

    fn local_now(&self) -> NaiveTime {
        TOD::now() + self.hour_offset + self.minute_offset + self.second_offset + self.millis_offset
            - self.halt_offset
    }

    fn is_locked(&self) -> bool {
        *self.read_lock.borrow() || self.write_lock
    }

    fn lock_read(&self) {
        if !self.is_locked() {
            *self.read_lock.borrow_mut() = true;
            *self.locked_val.borrow_mut() = self.local_now();
        }
    }

    fn lock_write(&mut self) {
        if !self.write_lock {
            self.write_lock = true;
            self.last_halt_duration = Some(Instant::now());
        }
    }

    fn unlock_read(&self) {
        *self.read_lock.borrow_mut() = false;
    }

    fn unlock_write(&mut self) {
        self.write_lock = false;
        let offset = Duration::milliseconds(
            self.last_halt_duration
                .unwrap()
                .elapsed()
                .as_millis()
                .try_into()
                .unwrap(),
        );
        self.halt_offset = self.halt_offset.checked_add(&offset).unwrap();
        self.last_halt_duration = None;
    }

    fn time(&self) -> NaiveTime {
        if self.is_locked() {
            *self.locked_val.borrow()
        } else {
            self.local_now()
        }
    }

    /// Returns current hour in BCD format with AM/PM flag.
    /// Bits 0-3: lower digit of an hour (0-9)
    /// Bits 4-6: higher digit of an hour (0-1)
    /// Bit 7: AM (0) or PM (1)
    /// Reading hour freezes the clock reads until the next call of
    /// tenth() method. internally the time flow continues normally
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
        self.unlock_read();
        dec_to_bcd(t as u8)
    }

    /// Sets hour of TOD and halts the clock completely (no time flow) until the call
    /// of set_tenth
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

    /// Sets 1/10th of the seconf of TOD and unhalts the clock (if previously halted with
    /// the call of set_hour)
    pub fn set_tenth(&mut self, val: u8) {
        self.millis_offset = Duration::milliseconds(val as i64 * 10);
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
