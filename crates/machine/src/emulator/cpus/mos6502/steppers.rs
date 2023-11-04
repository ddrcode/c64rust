use genawaiter::{rc::Gen,Generator};
use corosensei::{Coroutine, CoroutineResult};

use crate::emulator::abstractions::{CPU, Addr};
use crate::emulator::cpus::mos6502::{ OperationDef, OPERATIONS };


pub type OpGen<'a> = Box<dyn Generator<Yield = (), Return = ()> + 'a>;
pub type Stepper = Coroutine<(),(),bool>;

pub fn get_stepper(op: &OperationDef) -> Stepper {
    nop()
}

pub fn nop() -> Stepper {
    Coroutine::new(|yielder, input| {
        for _ in 0..7 {
            yielder.suspend(());
        }
        false
    })
}

// ----------------------------------------------------------------------
// Absolute addressing

pub fn abs_read(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let hi = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let (val, _) = read_from_addr(cpu, lo, hi);
        cpu.execute(val);
        co.yield_(()).await;
    }))
}

pub fn abs_write(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let hi = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let val = cpu.execute(0);
        cpu.write_byte(addr(lo, hi), val);
        co.yield_(()).await;
    }))
}

pub fn abs_rmw(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let hi = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let (val, addr) = read_from_addr(cpu, lo, hi);
        co.yield_(()).await;

        let new_val = write_and_exec(cpu, addr, val);
        co.yield_(()).await;

        cpu.write_byte(addr, new_val);
        co.yield_(()).await;
    }))
}

// ----------------------------------------------------------------------
// Zer-page addressing

pub fn zp_read(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let (val, _) = read_from_addr(cpu, lo, 0);
        cpu.execute(val);
        co.yield_(()).await;
    }))
}

pub fn zp_write(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let val = cpu.execute(0);
        cpu.write_byte(addr(lo, 0), val);
        co.yield_(()).await;
    }))
}

pub fn zp_rmw(cpu: &mut impl CPU) -> OpGen {
    Box::new(Gen::new(|co| async move {
        let lo = read_and_inc_pc(cpu);
        co.yield_(()).await;

        let (val, addr) = read_from_addr(cpu, lo, 0);
        co.yield_(()).await;

        let new_val = write_and_exec(cpu, addr, val);
        co.yield_(()).await;

        cpu.write_byte(addr, new_val);
        co.yield_(()).await;
    }))
}

// ----------------------------------------------------------------------
// utils

fn addr(lo: u8, hi: u8) -> Addr {
    u16::from_le_bytes([lo, hi])
}

fn read_and_inc_pc(cpu: &mut impl CPU) -> u8 {
    let val = cpu.read_byte(cpu.pc());
    cpu.inc_pc();
    val
}

fn read_from_addr(cpu: &impl CPU, lo: u8, hi: u8) -> (u8, Addr) {
    let addr = u16::from_le_bytes([lo, hi]);
    let val = cpu.read_byte(addr);
    (val, addr)
}

fn write_and_exec(cpu: &mut impl CPU, addr: Addr, val: u8) -> u8 {
    cpu.write_byte(addr, val);
    cpu.execute(val)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stepper() {
        let mut stepper = nop();
        match stepper.resume(()) {
            CoroutineResult::Yield(()) => {},
            CoroutineResult::Return(_) => {},
        };
    }
}

