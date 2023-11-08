mod address_mode;
mod mnemonic;
mod opcodes_def;
mod opcodes_impl;
mod operand;
mod operation;
mod operation_def;
mod steppers;

use std::collections::HashMap;

pub use {
    address_mode::*,
    mnemonic::*,
    opcodes_def::*,
    opcodes_impl::*,
    operand::*,
    operation::*,
    operation_def::*,
    steppers::*,
};

pub type OpsMap = HashMap<u8, OperationDef>;
