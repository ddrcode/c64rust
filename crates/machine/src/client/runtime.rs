use crate::machine::{Machine, MachineStatus::*};
use crate::utils::lock;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Interval between IRQs in [ms]
/// The value is specific for PAL systems. On NTSC systems the value
/// was 1/60s.
/// See:
/// https://dustlayer.com/machine-coding-tutorials/2013/4/8/episode-2-3-did-i-interrupt-you
const IRQ_INTERVAL: Duration = Duration::from_millis(20);

pub struct Runtime<M: Machine> {
    mutex: Arc<Mutex<M>>,
    irq_time: Instant,
    cycles: u128,
}

impl<M: Machine> Runtime<M> {
    pub fn new(mutex: Arc<Mutex<M>>) -> Self {
        Runtime {
            mutex: mutex,
            irq_time: Instant::now(),
            cycles: 0,
        }
    }

    fn irq_loop(&mut self) {
        if self.irq_time.elapsed() > IRQ_INTERVAL {
            self.irq_time = Instant::now();
            let mut machine = lock::<M>(&self.mutex);
            machine.irq();
        }
    }

    pub fn machine_loop(&mut self) {
        let mut status = Running;

        lock::<M>(&self.mutex).set_status(Running);
        while status != Stopped {
            {
                let mut machine = lock::<M>(&self.mutex);
                if status == Debug {
                    continue;
                }
                if let Some(max_cycles) = machine.get_config().max_cycles {
                    if self.cycles > max_cycles {}
                }
                machine.pre_next();
                machine.next();
                machine.post_next();
                // if let Some(on_next) = machine.get_events().on_next {
                //     on_next(&mut *machine, &self.cycles);
                // }
                if let Some(addr) = machine.get_config().exit_on_addr {
                    if machine.PC() == addr {
                        machine.debug();
                    }
                }
                status = machine.get_status();
            }
            self.irq_loop();
            self.cycles = self.cycles.wrapping_add(1);
        }
    }

    pub fn on_irq(cb: impl Fn(M)) {}
}
