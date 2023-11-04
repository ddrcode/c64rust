use genawaiter::{rc::Gen, Generator};

use crate::emulator::abstractions::{CPU, Addr};

pub type OpGen<'a> = Box<dyn Generator<Yield = (), Return = ()> + 'a>;

pub fn nop() -> OpGen<'static> {
    Box::new(Gen::new(|co| async move {
        for _ in 0..7 {
            co.yield_(()).await;
        }
    }))
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

