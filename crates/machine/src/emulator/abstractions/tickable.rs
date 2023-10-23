use std::cell::RefCell;

use crate::machine::Cycles;

pub trait Tickable {
    fn tick(&self);

    fn tick_times(&self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }
}

pub struct Ticker {
    cycles: RefCell<Cycles>,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            cycles: RefCell::new(0),
        }
    }

    pub fn cycles(&self) -> Cycles {
        *self.cycles.borrow()
    }
}

impl Tickable for Ticker {
    fn tick(&self) {
        let c = self.cycles().wrapping_add(1);
        *self.cycles.borrow_mut() = c;
    }
}
