mod address_mode;
mod cpu;
mod mnemonic;
mod opcodes_def;
mod opcodes_impl;
mod operand;
mod operation;
mod operation_def;
mod processor_status;
mod registers;

use std::collections::HashMap;

pub use {
    address_mode::AddressMode, cpu::MOS6510, mnemonic::Mnemonic, opcodes_def::define_operations,
    opcodes_impl::execute_operation, operand::Operand, operation::Operation,
    operation_def::OperationDef, processor_status::ProcessorStatus, registers::Registers,
};

pub type OpsMap = HashMap<u8, OperationDef>;
