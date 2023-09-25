use super::*;

pub struct EventStorage<T: Event + ?Sized> {
    events: Vec<EventFn<T>>,
}

impl<T: Event + ?Sized> EventStorage<T> {
    pub fn new() -> Self {
        EventStorage { events: Vec::new() }
    }

    pub fn push(&mut self, b: EventFn<T>) {
        self.events.push(b);
    }

    pub fn emit(&self, event: &T) {
        self.events.iter().for_each(|listener| {
            (listener.cb)(event);
        });
    }
}

pub struct EventFn<T: Event + ?Sized> {
    cb: Box<dyn Fn(&T)>,
}

impl<T: Event + ?Sized> EventFn<T> {
    pub fn new<F>(f: F) -> EventFn<T>
    where
        F: Fn(&T) + 'static,
    {
        EventFn { cb: Box::new(f) }
    }
}

pub struct EventEmitter {
    pub cpu_events: EventStorage<CpuStateChangeEvent>,
    pub memory_events: EventStorage<MemCellChangeEvent>,
}

impl EventEmitter {
    pub fn new() -> Self {
        EventEmitter {
            cpu_events: EventStorage::<CpuStateChangeEvent>::new(),
            memory_events: EventStorage::<MemCellChangeEvent>::new(),
        }
    }
}
