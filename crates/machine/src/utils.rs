use std::sync::{Arc, Mutex, MutexGuard};

pub fn lock<T: ?Sized>(arc: &Arc<Mutex<T>>) -> MutexGuard<T> {
    match arc.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// Compares two vectors
/// source: https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn if_else<T>(cond: bool, val1: T, val2: T) -> T {
    if cond {
        val1
    } else {
        val2
    }
}

pub fn dec_to_bcd(val: u8) -> u8 {
    ((val / 10) << 4) + (val % 10)
}

pub fn bcd_to_dec(val: u8) -> u8 {
    ((val & 0xf0) >> 4) * 10 + (val & 0x0f)
}
