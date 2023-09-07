mod mnemonic;
mod operations;
mod registers;
mod processor_status;
mod address_mode;
mod cpu;
mod operand;
mod operation_def;
mod operation;

use std::collections::HashMap;

pub use {
    mnemonic::Mnemonic,
    registers::Registers,
    operations::define_operations,
    processor_status::ProcessorStatus,
    address_mode::AddressMode,
    cpu::MOS6510,
    operand::Operand,
    operation_def::{ OperationDef, OpFn },
    operation::Operation
};

pub type OpsMap = HashMap<u8, OperationDef>;
