use std::cell::RefCell;
use std::rc::Rc;

use crate::emulator::cpus::CpuState;
use crate::emulator::cpus::mos6502::AddressMode::*;
use corosensei::{Coroutine, CoroutineResult};
use genawaiter::{rc::Gen, Generator};

use crate::emulator::abstractions::{Addr, CPU};
use crate::emulator::cpus::mos6502::{OperationDef, OPERATIONS};

use super::execute_operation;

pub type OpGen<'a> = Box<dyn Generator<Yield = (), Return = ()> + 'a>;
pub type Input = Rc<RefCell<CpuState>>;
pub type Stepper = Coroutine<Input, (), bool>;

pub fn get_stepper(op: &OperationDef) -> Option<Stepper> {
    use crate::emulator::cpus::mos6502::mnemonic::Mnemonic::*;
    match op.mnemonic {
        NOP => Some(nop()),
        LDA | LDX | LDY | EOR | AND | ORA | ADC | SBC | CMP | BIT => Some(read_stepper(op.clone())),
        _ => None,
    }
}

pub fn nop() -> Stepper {
    Coroutine::new(|yielder, _input: Input| {
        yielder.suspend(());
        yielder.suspend(());
        false
    })
}

fn read_stepper(op: OperationDef) -> Stepper {
    Coroutine::new(move |yielder, cpu: Input| {
        let lo = read_and_inc_pc(&cpu);
        yielder.suspend(());

        let hi = if op.address_mode == Absolute {
            let hi = read_and_inc_pc(&cpu);
            yielder.suspend(());
            hi
        } else {
            0
        };

        let (val, _) = read_from_addr(&cpu, lo, hi);
        // cpu.borrow_mut().execute(val);
        execute_operation(&mut cpu.borrow_mut(), &op, val);
        yielder.suspend(());

        false
    })
}

// // ----------------------------------------------------------------------
// // Absolute addressing
//
// pub fn abs_read(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let hi = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let (val, _) = read_from_addr(cpu, lo, hi);
//         cpu.execute(val);
//         co.yield_(()).await;
//     }))
// }
//
// pub fn abs_write(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let hi = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let val = cpu.execute(0);
//         cpu.write_byte(addr(lo, hi), val);
//         co.yield_(()).await;
//     }))
// }
//
// pub fn abs_rmw(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let hi = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let (val, addr) = read_from_addr(cpu, lo, hi);
//         co.yield_(()).await;
//
//         let new_val = write_and_exec(cpu, addr, val);
//         co.yield_(()).await;
//
//         cpu.write_byte(addr, new_val);
//         co.yield_(()).await;
//     }))
// }
//
// // ----------------------------------------------------------------------
// // Zer-page addressing
//
// pub fn zp_read(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let (val, _) = read_from_addr(cpu, lo, 0);
//         cpu.execute(val);
//         co.yield_(()).await;
//     }))
// }
//
// pub fn zp_write(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let val = cpu.execute(0);
//         cpu.write_byte(addr(lo, 0), val);
//         co.yield_(()).await;
//     }))
// }
//
// pub fn zp_rmw(cpu: &mut impl CPU) -> OpGen {
//     Box::new(Gen::new(|co| async move {
//         let lo = read_and_inc_pc(cpu);
//         co.yield_(()).await;
//
//         let (val, addr) = read_from_addr(cpu, lo, 0);
//         co.yield_(()).await;
//
//         let new_val = write_and_exec(cpu, addr, val);
//         co.yield_(()).await;
//
//         cpu.write_byte(addr, new_val);
//         co.yield_(()).await;
//     }))
// }

// ----------------------------------------------------------------------
// utils

fn addr(lo: u8, hi: u8) -> Addr {
    u16::from_le_bytes([lo, hi])
}

fn read_and_inc_pc(cpu_ref: &Input) -> u8 {
    let cpu = cpu_ref.borrow();
    let val = cpu.read_byte(cpu.pc());
    drop(cpu);
    cpu_ref.borrow_mut().inc_pc();
    val
}

fn read_from_addr(cpu: &Input, lo: u8, hi: u8) -> (u8, Addr) {
    let addr = u16::from_le_bytes([lo, hi]);
    let val = cpu.borrow().read_byte(addr);
    (val, addr)
}

fn write_and_exec(cpu: &mut impl CPU, addr: Addr, val: u8) -> u8 {
    cpu.write_byte(addr, val);
    cpu.execute(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_stepper() {
    //     let mut stepper = nop();
    //     match stepper.resume(()) {
    //         CoroutineResult::Yield(()) => {},
    //         CoroutineResult::Return(_) => {},
    //     };
    // }
}
