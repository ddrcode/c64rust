use std::sync::{Arc, Mutex, MutexGuard};

pub fn lock<T>(arc: &Arc<Mutex<T>>) -> MutexGuard<T> {
    match arc.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}
