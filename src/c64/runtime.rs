use std::sync::{Arc, Mutex};
use super::C64;
use std::thread;
use std::time;


/// Interval between IRQs in [ms]
/// The value is specific for PAL systems. On NTSC systems the value
/// was 1/60s. 
/// See:
/// https://dustlayer.com/c64-coding-tutorials/2013/4/8/episode-2-3-did-i-interrupt-you
const IRQ_INTERVAL: u32 = 20;

type C64Arc = Arc<Mutex<C64>>;

pub fn irq_loop(c64: C64Arc) {
    loop {
        thread::sleep_ms(IRQ_INTERVAL);
        c64.lock().unwrap().machine.irq();
    }
}


pub fn machine_loop(c64mutex: Arc<Mutex<C64>>) {
    let mut cycles = 0u64;
    let sleep_time = time::Duration::from_nanos(1);
    loop {
        {
            let mut c64 = c64mutex.lock().unwrap();
            if let Some(max_cycles) = c64.machine.config.max_cycles {
                if cycles > max_cycles {
                    break;
                }
            }
            if !c64.machine.next() {
                break;
            };
            if let Some(on_next) = c64.machine.events.on_next {
                on_next(&mut c64.machine, &cycles);
            }
            if let Some(addr) = c64.machine.config.exit_on_addr {
                if c64.machine.PC() == addr {
                    break;
                }
            }
        }
        // thread::sleep(sleep_time);
        cycles += 1;
    }
}
