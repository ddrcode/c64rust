use super::{ AddressMode, Operation };
use crate::c64::C64;

// source: http://6502.org/tutorials/6502opcodes.html
#[derive(Copy, Clone)]
pub enum Mnemonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

pub enum ProcessorFlag {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    Overflow = 6,
    Negative = 7
}

pub type OpFn = fn(&Operation, &mut C64);

#[derive(Copy, Clone)]
pub struct OperationDef {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub cycles: u8,
    pub page_boundary_cycle: bool,
    // length: u8,
    pub address_mode: AddressMode,
    pub function: OpFn
}

