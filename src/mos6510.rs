mod address_mode;
mod cpu;
mod mnemonic;
mod operand;
mod operation;
mod operation_def;
mod operations;
mod operations_impl;
mod processor_status;
mod registers;

use std::collections::HashMap;

pub use {
    address_mode::AddressMode, cpu::MOS6510, mnemonic::Mnemonic, operand::Operand,
    operation::Operation, operation_def::OperationDef, operations::define_operations,
    operations_impl::execute_operation, processor_status::ProcessorStatus, registers::Registers,
};

pub type OpsMap = HashMap<u8, OperationDef>;
