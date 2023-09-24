use crate::machine::{Machine, MachineStatus::*};
use crate::utils::lock;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Interval between IRQs in [ms]
/// The value is specific for PAL systems. On NTSC systems the value
/// was 1/60s.
/// See:
/// https://dustlayer.com/machine-coding-tutorials/2013/4/8/episode-2-3-did-i-interrupt-you
const IRQ_INTERVAL: Duration = Duration::from_millis(20);

pub fn irq_loop<M: Machine>(mutex: Arc<Mutex<M>>) {
    loop {
        thread::sleep(IRQ_INTERVAL);
        {
            let mut machine = lock::<M>(&mutex);
            match machine.get_status() {
                Stopped => break,
                Debug => continue,
                _ => {}
            };
            machine.irq();
        }
    }
}

pub fn machine_loop<M: Machine>(mutex: Arc<Mutex<M>>) {
    let mut cycles = 0u128;
    let mut status = Running;
    lock::<M>(&mutex).set_status(Running);
    while status != Stopped {
        {
            let mut machine = lock::<M>(&mutex);
            if status == Debug {
                continue;
            }
            if let Some(max_cycles) = machine.get_config().max_cycles {
                if cycles > max_cycles {}
            }
            machine.next();
            if let Some(on_next) = machine.get_events().on_next {
                on_next(&mut *machine, &cycles);
            }
            if let Some(addr) = machine.get_config().exit_on_addr {
                if machine.PC() == addr {
                    machine.debug();
                }
            }
            status = machine.get_status();
        }
        cycles = cycles.wrapping_add(1);
    }
}
