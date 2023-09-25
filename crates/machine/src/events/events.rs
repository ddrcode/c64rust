use crate::machine::Addr;
use crate::mos6502::Registers;
use std::fmt;

pub trait Event {}

#[derive(Debug)]
pub struct CpuStateChangeEvent {
    pub registers: Registers,
}
impl CpuStateChangeEvent {
    pub fn new(r: Registers) -> Self {
        CpuStateChangeEvent { registers: r }
    }
}
impl Event for CpuStateChangeEvent {}

pub struct MemCellChangeEvent {
    pub new_val: u8,
    pub old_val: u8,
    pub addr: Addr,
}
impl Event for MemCellChangeEvent {}

pub struct NextEvent {}
impl Event for NextEvent {}
