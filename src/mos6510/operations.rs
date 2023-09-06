use std::num::Wrapping;
use super::{
    Operation, OperationDef, Mnemonic, Mnemonic::*, AddressMode, AddressMode::*, OpFn, OpsMap, Operand,
    ProcessorStatus
};
use crate::c64::{ C64, RegSetter };

// sources:
// https://c64os.com/post/6502instructions
// https://www.masswerk.at/6502/6502_instruction_set.html
// http://www.6502.org/tutorials/6502opcodes.html

pub fn define_operations(o: &mut OpsMap) -> &OpsMap {
    type OpData = (u8, u8, bool, AddressMode);

    let mut ops3 = OpsMap::new();

    let mut add_op = |mnemonic: Mnemonic, opcode: u8, cycles: u8, boundary: bool, am: AddressMode, opfn: OpFn| {
        o.insert(opcode, OperationDef {
            opcode: opcode,
            mnemonic: mnemonic,
            cycles: cycles,
            page_boundary_cycle: boundary,
            address_mode: am,
            function: opfn
        });
    };


    let mut add_group = |mnemonic: Mnemonic, opfn: OpFn, data: &[OpData]| {
        for op in data.iter() {
            let (opcode, cycles, boundary, am) = *op;
            add_op(mnemonic, opcode, cycles, boundary, am, opfn);
        }
    };

    let mut add_functional_group = |cycles: u8, boundary: bool, am: AddressMode, opfn: OpFn, data: &[(Mnemonic, u8)]| {
        for op in data.iter() {
            let (mnemonic, opcode) = *op;
            ops3.insert(opcode, OperationDef {
                opcode: opcode,
                mnemonic: mnemonic,
                cycles: cycles,
                page_boundary_cycle: boundary,
                address_mode: am,
                function: opfn
            });
        }
    };

    add_group(ADC, op_adc, &[
        (0x69, 2, false, Immediate),
        (0x65, 3, false, ZeroPage)
    ]);

    add_group(AND, op_bitwise, &[
        (0x29, 2, false, Immediate),
        (0x25, 3, false, ZeroPage),
        (0x35, 4, false, ZeroPageX),
        (0x2d, 4, false, Absolute),
        (0x3d, 4, true, AbsoluteX),
        (0x39, 4, true, AbsoluteY),
        (0x21, 6, false, IndirectX),
        (0x31, 5, true, IndirectY),
    ]);

    add_group(CMP, op_compare, &[
        (0xc9, 2, false, Immediate),
        (0xc5, 3, false, ZeroPage),
        (0xd5, 4, false, ZeroPageX),
        (0xcd, 4, false, Absolute),
        (0xdd, 4, true, AbsoluteX),
        (0xd9, 4, true, AbsoluteY),
        (0xc1, 6, false, IndirectX),
        (0xd1, 5, true, IndirectY),
    ]);

    add_group(CPX, op_compare, &[
        (0xe0, 2, false, Immediate),
        (0xe4, 3, false, ZeroPage),
        (0xec, 4, false, Absolute),
    ]);

    add_group(CPY, op_compare, &[
        (0xc0, 2, false, Immediate),
        (0xc4, 3, false, ZeroPage),
        (0xcc, 4, false, Absolute),
    ]);

    add_group(DEC, op_incdec_mem, &[
        (0xc6, 5, false, ZeroPage),
        (0xd6, 6, false, ZeroPageX),
        (0xce, 6, false, Absolute),
        (0xde, 7, false, AbsoluteX),
    ]);

    add_group(EOR, op_bitwise, &[
        (0x49, 2, false, Immediate),
        (0x45, 3, false, ZeroPage),
        (0x55, 4, false, ZeroPageX),
        (0x4d, 4, false, Absolute),
        (0x5d, 4, true, AbsoluteX),
        (0x59, 4, true, AbsoluteY),
        (0x41, 6, false, IndirectX),
        (0x51, 5, true, IndirectY),
    ]);

    add_group(INC, op_incdec_mem, &[
        (0xe6, 5, false, ZeroPage),
        (0xf6, 6, false, ZeroPageX),
        (0xee, 6, false, Absolute),
        (0xfe, 7, false, AbsoluteX),
    ]);

    add_group(LDA, op_load, &[
        (0xa9, 2, false, Immediate),
        (0xa5, 3, false, ZeroPage),
        (0xb5, 4, false, ZeroPageX),
        (0xad, 4, false, Absolute),
        (0xbd, 4, true, AbsoluteX),
        (0xb9, 4, true, AbsoluteY),
        (0xa1, 6, false, IndirectX),
        (0xb1, 5, true, IndirectY),
    ]);

    add_group(LDX, op_load, &[
        (0xa2, 2, false, Immediate),
        (0xa6, 3, false, ZeroPage),
        (0xb6, 4, false, ZeroPageY),
        (0xae, 4, false, Absolute),
        (0xbe, 4, true, AbsoluteY),
    ]);

    add_group(LDY, op_load, &[
        (0xa0, 2, false, Immediate),
        (0xa4, 3, false, ZeroPage),
        (0xb4, 4, false, ZeroPageX),
        (0xac, 4, false, Absolute),
        (0xbc, 4, true, AbsoluteX),
    ]);

    add_group(NOP, op_nop, &[
        (0xea, 2, false, Implicit),
        (0x80, 2, false, Immediate), // illegal
        (0x04, 3, false, ZeroPage), // illegal
        (0x34, 4, false, ZeroPageX), // illegal
    ]);

    add_group(ORA, op_bitwise, &[
        (0x09, 2, false, Immediate),
        (0x05, 3, false, ZeroPage),
        (0x15, 4, false, ZeroPageX),
        (0x0d, 4, false, Absolute),
        (0x1d, 4, true, AbsoluteX),
        (0x19, 4, true, AbsoluteY),
        (0x01, 6, false, IndirectX),
        (0x11, 5, true, IndirectY),
    ]);

    add_group(ROL, op_rotate, &[
        (0x2a, 2, false, Accumulator),
        (0x26, 5, false, ZeroPage),
        (0x36, 6, false, ZeroPageX),
        (0x2e, 6, false, Absolute),
        (0x3e, 7, false, AbsoluteX)
    ]);

    add_group(ROR, op_rotate, &[
        (0x6a, 2, false, Accumulator),
        (0x66, 5, false, ZeroPage),
        (0x76, 6, false, ZeroPageX),
        (0x6e, 6, false, Absolute),
        (0x7e, 7, false, AbsoluteX)
    ]);

    add_group(SBC, op_sbc, &[
        (0xe9, 2, false, Immediate),
        (0xe5, 3, false, ZeroPage),
        (0xf5, 4, false, ZeroPageX),
        (0xed, 3, false, Absolute),
        (0xe1, 6, false, IndirectX),
    ]);

    add_group(STA, op_store, &[
        (0x85, 3, false, ZeroPage),
        (0x95, 4, false, ZeroPageX),
        (0x8d, 4, false, Absolute),
        (0x9d, 5, false, AbsoluteX),
        (0x99, 5, false, AbsoluteY),
        (0x81, 6, false, IndirectX),
        (0x91, 6, false, IndirectY),
    ]);

    add_group(STX, op_store, &[
        (0x86, 3, false, ZeroPage),
        (0x96, 4, false, ZeroPageY),
        (0x8e, 4, false, Absolute),
    ]);

    add_group(STY, op_store, &[
        (0x84, 3, false, ZeroPage),
        (0x94, 4, false, ZeroPageX),
        (0x8c, 4, false, Absolute),
    ]);

    add_group(JMP, op_jmp, &[
        (0x4c, 3, false, Absolute),
        (0x6c, 5, false, Indirect),
    ]);

    // branching
    add_functional_group(2, true, Relative, op_branch,
        &[(BCC, 0x90), (BCS, 0xb0), (BEQ, 0xf0), (BNE, 0xd0), (BPL, 0x10), (BMI, 0x30)]);

    // flag set/unset
    add_functional_group(2, false, Implicit, op_flag,
        &[(CLC, 0x18), (SEC, 0x38), (CLD, 0xd8), (SED, 0xf8), (CLI, 0x58), (SEI, 0x78), (CLV, 0xb8)]);

    // transfer (between registers)
    add_functional_group(2, false, Implicit, op_transfer,
        &[(TAX, 0xaa), (TAY, 0xa8), (TXA, 0x8a), (TYA, 0x98), (TXS, 0x9a), (TSX, 0xba)]);

    // stack push
    add_functional_group(3, false, Implicit, op_push,
        &[(PHA, 0x48), (PHP, 0x08)]);
    //
    // incrementation / decrementation
    add_functional_group(2, false, Implicit, op_incdec_reg,
        &[(DEX, 0xca), (DEY, 0x88), (INX, 0xe8), (INY, 0xc8)]);

    // jumps and returns
    add_op(JSR, 0x20, 6, false, Absolute, op_jsr);
    add_op(RTS, 0x60, 6, false, Implicit, op_rts);

    // other
    add_op(BRK, 0x00, 7, false, Implicit, op_brk);

    o.extend(ops3);
    o
}

// ----------------------------------------------------------------------
// helpers

fn get_val(op: &Operation, c64: &C64) -> Option<u8> {
    if let Some(addr) = op.address { Some(c64.mem.get_byte(addr)) }
    else if op.def.address_mode == Immediate { op.operand.as_ref().unwrap().get_byte() }
    else if op.def.address_mode == Accumulator { Some(c64.A8()) }
    else { None }
}

fn set_val(val: u8, op: &Operation, c64: &mut C64) {
    if let Some(addr) = op.address { c64.mem.set_byte(addr, val) }
    else if op.def.address_mode == Accumulator { c64.set_A(val) }
    else { panic!("Can't set value for address mode {}", op.def.address_mode) };
}

fn store_byte(val: u8, op: &Operation, c64: &mut C64) -> u8 {
    c64.mem.set_byte(op.address.unwrap(), val);
    op.def.cycles
}

fn set_flags(flags: &str, vals: &[bool], c64: &mut C64) {
    let chars  = String::from(flags);
    if chars.len() != vals.len() { panic!("Incorrect args length in set_flags") };
    for (i, ch) in chars.chars().enumerate() {
        match ch {
            'Z' => c64.cpu.registers.status.zero = vals[i],
            'N' => c64.cpu.registers.status.negative = vals[i],
            'C' => c64.cpu.registers.status.carry = vals[i],
            'I' => c64.cpu.registers.status.interrupt_disable = vals[i],
            'D' => c64.cpu.registers.status.decimal_mode = vals[i],
            'V' => c64.cpu.registers.status.overflow = vals[i],
            _ => panic!("Unrecognized flag symbol: {}", ch)
        };
    }
}

fn set_nz_flags(val: u8, c64: &mut C64) {
    c64.cpu.registers.status.negative = neg(val);
    c64.cpu.registers.status.zero = zero(val);
}

fn neg(val: u8) -> bool { val & 0x80 > 0 }
fn zero(val: u8) -> bool { val == 0 }

// see https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
fn overflow(in1: u8, in2: u8, result: u8) -> bool {
    ((in1^result) & (in2^result) & 0x80) > 0
}

// ----------------------------------------------------------------------
// implementation of operations

fn op_adc(op: &Operation, c64: &mut C64) -> u8 {
    let a = c64.A8();
    let val = get_val(op, c64).unwrap();
    let carry = u16::from(c64.P().carry);
    let sum = c64.A16() + val as u16 + carry;
    c64.set_A((sum & 0xff) as u8);
    let res = c64.A8();
    set_flags("NZCV", &[neg(res), zero(res), sum>0xff, overflow(a, val, res)], c64);
    op.def.cycles
}

fn op_branch(op: &Operation, c64: &mut C64) -> u8 {
    let branch: bool = match op.def.mnemonic {
        BCC => !c64.P().carry,
        BCS => c64.P().carry,
        BNE => !c64.P().zero,
        BEQ => c64.P().zero,
        BPL => !c64.P().negative,
        BMI => c64.P().negative,
        _ => panic!("{} is not a branch operation", op.def.mnemonic)
    };
    if branch {
        c64.cpu.registers.counter = op.address.unwrap();
        return op.def.cycles + 1 // TODO consider page change
    }
    op.def.cycles
}

fn op_brk(op: &Operation, c64: &mut C64) -> u8 {
    set_flags("I", &[true], c64);
    op.def.cycles
}

// TODO add cycle for page change
fn op_compare(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    let reg = match op.def.mnemonic {
        CMP => c64.A8(),
        CPX => c64.X8(),
        CPY => c64.Y8(),
        _ => panic!("{} is not a compare operation", op.def.mnemonic)
    };
    // TODO fix N flag: it should be a sign bit of the result
    // TODO check C flag - whether > or >= operator should be used
    set_flags("NZC", &[false, reg==val, reg >= val], c64);
    op.def.cycles
}

fn op_incdec_mem(op: &Operation, c64: &mut C64) -> u8 {
    let mut val = Wrapping(get_val(op, c64).unwrap());
    match op.def.mnemonic {
        DEC => val -= 1,
        INC => val += 1,
        _ => panic!("{} is not a inc/dec (mem) operation", op.def.mnemonic)
    };
    set_val(val.0, op, c64);
    set_nz_flags(val.0, c64);
    op.def.cycles
}

fn op_incdec_reg(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        DEX => c64.cpu.registers.x -= 1,
        DEY => c64.cpu.registers.y -= 1,
        INX => c64.cpu.registers.x += 1,
        INY => c64.cpu.registers.y += 1,
        _ => panic!("{} is not a inc/dec operation", op.def.mnemonic)
    };
    let val = match op.def.mnemonic {
        DEX | INX => c64.X8(),
        DEY | INY => c64.Y8(),
        _ => panic!("{} is not a inc/dec operation", op.def.mnemonic)
    };
    set_nz_flags(val, c64);
    op.def.cycles
}

// TODO add cycle for page change
fn op_bitwise(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    match op.def.mnemonic {
        AND => c64.cpu.registers.accumulator &= val,
        ORA => c64.cpu.registers.accumulator |= val,
        EOR => c64.cpu.registers.accumulator ^= val,
        _ => panic!("{} is not a bitwise operation", op.def.mnemonic)
    };
    set_nz_flags(c64.A8(), c64);
    op.def.cycles
}

fn op_flag(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        CLC => c64.cpu.registers.status.carry = false,
        SEC => c64.cpu.registers.status.carry = true,
        CLI => c64.cpu.registers.status.interrupt_disable = false,
        SEI => c64.cpu.registers.status.interrupt_disable = true,
        CLD => c64.cpu.registers.status.decimal_mode = false,
        SED => c64.cpu.registers.status.decimal_mode = true,
        CLV => c64.cpu.registers.status.overflow = false,
        _ => panic!("{} is not a flag set/unset operation", op.def.mnemonic)
    };
    op.def.cycles
}

// TODO: JMP doesn't support cross-page
// For example, JMP ($30FF) will read the vector low byte from $30FF, but it will read the vector
// high byte from $3000, NOT from $4000
// see JMP on https://c64os.com/post/6502instructions
fn op_jmp(op: &Operation, c64: &mut C64) -> u8 {
    c64.cpu.registers.counter = op.address.unwrap();
    op.def.cycles
}

fn op_jsr(op: &Operation, c64: &mut C64) -> u8 {
    let pc = c64.PC();
    c64.push((pc >> 8) as u8);
    c64.push((pc & 0x00ff) as u8);
    c64.cpu.registers.counter = op.address.unwrap();
    op.def.cycles
}

// FIXME add cycle for crossing page boundary
fn op_load(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    match op.def.mnemonic {
        LDA => c64.set_A(val),
        LDX => c64.set_X(val),
        LDY => c64.set_Y(val),
        _ => panic!("{} is not a load operation", op.def.mnemonic)
    };
    set_nz_flags(val, c64);
    op.def.cycles
}

fn op_nop(op: &Operation, c64: &mut C64) -> u8 {
    op.def.cycles
}

fn op_push(op: &Operation, c64: &mut C64) -> u8 {
    let val = match op.def.mnemonic {
        PHA => c64.A8(),
        PHP => u8::from(&c64.P()),
        _ => panic!("{} is not a push operation", op.def.mnemonic)
    };
    c64.mem.set_byte(c64.stack_addr(), val);
    c64.cpu.registers.stack -= 1;
    op.def.cycles
}

fn op_rotate(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    let c = if c64.P().carry { 0xff } else { 0 };
    let (new_val, mask, carry) = match op.def.mnemonic {
        ROL => (val << 1, c & 1, val & 0b10000000 > 0),
        ROR => (val >> 1, c & 0b10000000, val & 1 > 0),
        _ => panic!("{} is not a rotate operation", op.def.mnemonic)
    };
    set_val(new_val | mask, op, c64);
    set_flags("NZC", &[false, new_val | mask == 0, carry], c64);
    op.def.cycles
}

fn op_rts(op: &Operation, c64: &mut C64) -> u8 {
    c64.cpu.registers.counter = (c64.pop() as u16 | (c64.pop() as u16) << 8) as u16;
    op.def.cycles
}

// https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
// http://retro.hansotten.nl/uploads/mag6502/sbc_tsx_txs_instructions.pdf
fn op_sbc(op: &Operation, c64: &mut C64) -> u8 {
    let a = c64.A8();
    let val = get_val(op, c64).unwrap();
    let carry = u16::from(!c64.P().carry);
    let sum = c64.A16() + (!val) as u16 + carry;
    c64.set_A((sum & 0xff) as u8);
    let res = c64.A8();
    set_flags("NZCV", &[neg(res), zero(res), sum>0xff, overflow(a, !val, res)], c64);
    op.def.cycles
}

fn op_store(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        STA => store_byte(c64.A8(), op, c64),
        STX => store_byte(c64.X8(), op, c64),
        STY => store_byte(c64.Y8(), op, c64),
        _ => panic!("{} is not a store operation", op.def.mnemonic)
    }
}

fn op_transfer(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        TAX => c64.cpu.registers.x = c64.A(),
        TAY => c64.cpu.registers.y = c64.A(),
        TXA => c64.set_A(c64.X()),
        TYA => c64.set_A(c64.Y()),
        TXS => c64.set_SC(c64.X()),
        TSX => c64.set_X(c64.SC()),
        _ => panic!("{} is not a transfer operation", op.def.mnemonic)
    };
    if op.def.mnemonic != TXS { // TXS doesn't change any flag
        set_nz_flags(c64.A8(), c64);
    }
    op.def.cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils() {
        assert!(neg(0x80));
        assert!(zero(0));
    }
}
