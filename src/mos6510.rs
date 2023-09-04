mod models;
mod operations;
mod registers;
mod processor_status;
mod address_mode;
mod cpu;
mod operand;

use std::collections::HashMap;

pub use {
    models::{ Mnemonic, ProcessorFlag, Operation, OpFn },
    registers::Registers,
    operations::define_operations,
    processor_status::ProcessorStatus,
    address_mode::AddressMode,
    cpu::MOS6510,
    operand::Operand,
};

pub type OpsMap = HashMap<u8, Operation>;
