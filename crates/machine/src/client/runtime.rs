use crate::machine::{Machine, MachineStatus::*};
use crate::utils::lock;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Interval between IRQs in [ms]
/// The value is specific for PAL systems. On NTSC systems the value
/// was 1/60s.
/// See:
/// https://dustlayer.com/machine-coding-tutorials/2013/4/8/episode-2-3-did-i-interrupt-you
/// (it's always 1/60 of a second ,regardless whether it's PAL or NTSC)
const JIFFY: Duration = Duration::from_millis(1000 / 60);

pub struct Runtime<M: Machine> {
    mutex: Arc<Mutex<M>>,
    irq_time: Instant,
}

impl<M: Machine> Runtime<M> {
    pub fn new(mutex: Arc<Mutex<M>>) -> Self {
        Runtime {
            mutex,
            irq_time: Instant::now(),
        }
    }

    fn irq_loop(&mut self) {
        if self.irq_time.elapsed() > JIFFY {
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
                status = machine.get_status();
                if status == Debug {
                    continue;
                }
                machine.next();
                if let Some(addr) = machine.get_config().exit_on_addr {
                    if machine.PC() == addr {
                        machine.debug();
                    }
                }
                // status must be checked 2nd time after next() - in case of BRK
                status = machine.get_status();
            }
            self.irq_loop();
        }
    }
}
