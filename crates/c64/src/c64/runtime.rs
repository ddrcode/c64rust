use super::C64;
use machine::{utils::lock, Machine, MachineStatus};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Interval between IRQs in [ms]
/// The value is specific for PAL systems. On NTSC systems the value
/// was 1/60s.
/// See:
/// https://dustlayer.com/c64-coding-tutorials/2013/4/8/episode-2-3-did-i-interrupt-you
const IRQ_INTERVAL: Duration = Duration::from_millis(20);

pub fn irq_loop(c64mutex: Arc<Mutex<C64>>) {
    loop {
        thread::sleep(IRQ_INTERVAL);
        {
            let mut c64 = lock(&c64mutex);
            match c64.get_status() {
                MachineStatus::Stopped => break,
                MachineStatus::Debug => continue,
                _ => {}
            };
            c64.cia1.keyboard.cycle();
            c64.irq();
        }
    }
}

pub fn machine_loop(c64mutex: Arc<Mutex<C64>>) {
    let mut cycles = 0u64;
    let mut status = MachineStatus::Running;
    lock(&c64mutex).set_status(MachineStatus::Running);
    while status != MachineStatus::Stopped {
        {
            let mut c64 = lock(&c64mutex);
            status = *c64.get_status();
            if status == MachineStatus::Debug {
                continue;
            }
            if let Some(max_cycles) = c64.get_config().max_cycles {
                if cycles > max_cycles {
                    c64.stop();
                }
            }
            c64.next();
            if let Some(on_next) = c64.get_events().on_next {
                on_next(&mut *c64, &cycles);
            }
            if let Some(addr) = c64.get_config().exit_on_addr {
                if c64.PC() == addr {
                    c64.debug();
                }
            }
        }
        cycles += 1;
    }
}
