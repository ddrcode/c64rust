use super::{
    AddressMode, AddressMode::*, Mnemonic, Mnemonic::*, OpFn, Operation, OperationDef, OpsMap,
    ProcessorStatus,
};
use crate::machine::{Machine, RegSetter};
use std::num::Wrapping;

// sources:
// https://c64os.com/post/6502instructions
// https://www.masswerk.at/6502/6502_instruction_set.html
// http://www.6502.org/tutorials/6502opcodes.html

pub fn define_operations(o: &mut OpsMap) -> &OpsMap {
    type OpData = (u8, u8, bool, AddressMode);

    let mut ops3 = OpsMap::new();

    let mut add_op = |mnemonic: Mnemonic,
                      opcode: u8,
                      cycles: u8,
                      boundary: bool,
                      am: AddressMode,
                      opfn: OpFn| {
        if o.contains_key(&opcode) {
            panic!("Opcode {} already exists in the opsmap", opcode);
        }
        o.insert(
            opcode,
            OperationDef {
                opcode: opcode,
                mnemonic: mnemonic,
                cycles: cycles,
                page_boundary_cycle: boundary,
                address_mode: am,
                function: opfn,
            },
        );
    };

    let mut add_group = |mnemonic: Mnemonic, opfn: OpFn, data: &[OpData]| {
        for op in data.iter() {
            let (opcode, cycles, boundary, am) = *op;
            add_op(mnemonic, opcode, cycles, boundary, am, opfn);
        }
    };

    let mut add_functional_group =
        |cycles: u8, boundary: bool, am: AddressMode, opfn: OpFn, data: &[(Mnemonic, u8)]| {
            for op in data.iter() {
                let (mnemonic, opcode) = *op;
                ops3.insert(
                    opcode,
                    OperationDef {
                        opcode: opcode,
                        mnemonic: mnemonic,
                        cycles: cycles,
                        page_boundary_cycle: boundary,
                        address_mode: am,
                        function: opfn,
                    },
                );
            }
        };

    add_group(
        ADC,
        op_arithmetic,
        &[
            (0x69, 2, false, Immediate),
            (0x65, 3, false, ZeroPage),
            (0x75, 4, false, ZeroPageX),
            (0x6d, 4, false, Absolute),
            (0x7d, 4, true, AbsoluteX),
            (0x79, 4, true, AbsoluteY),
            (0x61, 6, false, IndirectX),
            (0x71, 5, true, IndirectY),
        ],
    );

    add_group(
        AND,
        op_bitwise,
        &[
            (0x29, 2, false, Immediate),
            (0x25, 3, false, ZeroPage),
            (0x35, 4, false, ZeroPageX),
            (0x2d, 4, false, Absolute),
            (0x3d, 4, true, AbsoluteX),
            (0x39, 4, true, AbsoluteY),
            (0x21, 6, false, IndirectX),
            (0x31, 5, true, IndirectY),
        ],
    );

    add_group(
        ASL,
        op_shift,
        &[
            (0x0a, 2, false, Accumulator),
            (0x06, 5, false, ZeroPage),
            (0x16, 6, false, ZeroPageX),
            (0x0e, 6, false, Absolute),
            (0x1e, 7, false, AbsoluteX),
        ],
    );

    add_group(
        BIT,
        op_bit,
        &[(0x24, 3, false, ZeroPage), (0x2c, 4, false, Absolute)],
    );

    add_group(
        CMP,
        op_compare,
        &[
            (0xc9, 2, false, Immediate),
            (0xc5, 3, false, ZeroPage),
            (0xd5, 4, false, ZeroPageX),
            (0xcd, 4, false, Absolute),
            (0xdd, 4, true, AbsoluteX),
            (0xd9, 4, true, AbsoluteY),
            (0xc1, 6, false, IndirectX),
            (0xd1, 5, true, IndirectY),
        ],
    );

    add_group(
        CPX,
        op_compare,
        &[
            (0xe0, 2, false, Immediate),
            (0xe4, 3, false, ZeroPage),
            (0xec, 4, false, Absolute),
        ],
    );

    add_group(
        CPY,
        op_compare,
        &[
            (0xc0, 2, false, Immediate),
            (0xc4, 3, false, ZeroPage),
            (0xcc, 4, false, Absolute),
        ],
    );

    add_group(
        DEC,
        op_incdec_mem,
        &[
            (0xc6, 5, false, ZeroPage),
            (0xd6, 6, false, ZeroPageX),
            (0xce, 6, false, Absolute),
            (0xde, 7, false, AbsoluteX),
        ],
    );

    add_group(
        EOR,
        op_bitwise,
        &[
            (0x49, 2, false, Immediate),
            (0x45, 3, false, ZeroPage),
            (0x55, 4, false, ZeroPageX),
            (0x4d, 4, false, Absolute),
            (0x5d, 4, true, AbsoluteX),
            (0x59, 4, true, AbsoluteY),
            (0x41, 6, false, IndirectX),
            (0x51, 5, true, IndirectY),
        ],
    );

    add_group(
        INC,
        op_incdec_mem,
        &[
            (0xe6, 5, false, ZeroPage),
            (0xf6, 6, false, ZeroPageX),
            (0xee, 6, false, Absolute),
            (0xfe, 7, false, AbsoluteX),
        ],
    );

    add_group(
        LDA,
        op_load,
        &[
            (0xa9, 2, false, Immediate),
            (0xa5, 3, false, ZeroPage),
            (0xb5, 4, false, ZeroPageX),
            (0xad, 4, false, Absolute),
            (0xbd, 4, true, AbsoluteX),
            (0xb9, 4, true, AbsoluteY),
            (0xa1, 6, false, IndirectX),
            (0xb1, 5, true, IndirectY),
        ],
    );

    add_group(
        LDX,
        op_load,
        &[
            (0xa2, 2, false, Immediate),
            (0xa6, 3, false, ZeroPage),
            (0xb6, 4, false, ZeroPageY),
            (0xae, 4, false, Absolute),
            (0xbe, 4, true, AbsoluteY),
        ],
    );

    add_group(
        LDY,
        op_load,
        &[
            (0xa0, 2, false, Immediate),
            (0xa4, 3, false, ZeroPage),
            (0xb4, 4, false, ZeroPageX),
            (0xac, 4, false, Absolute),
            (0xbc, 4, true, AbsoluteX),
        ],
    );

    add_group(
        LSR,
        op_shift,
        &[
            (0x4a, 2, false, Accumulator),
            (0x46, 5, false, ZeroPage),
            (0x56, 6, false, ZeroPageX),
            (0x4e, 6, false, Absolute),
            (0x5e, 7, false, AbsoluteX),
        ],
    );

    add_group(
        NOP,
        op_nop,
        &[
            (0xea, 2, false, Implicit),
            (0x80, 2, false, Immediate), // illegal
            (0x04, 3, false, ZeroPage),  // illegal
            (0x34, 4, false, ZeroPageX), // illegal
        ],
    );

    add_group(
        ORA,
        op_bitwise,
        &[
            (0x09, 2, false, Immediate),
            (0x05, 3, false, ZeroPage),
            (0x15, 4, false, ZeroPageX),
            (0x0d, 4, false, Absolute),
            (0x1d, 4, true, AbsoluteX),
            (0x19, 4, true, AbsoluteY),
            (0x01, 6, false, IndirectX),
            (0x11, 5, true, IndirectY),
        ],
    );

    add_group(
        ROL,
        op_rotate,
        &[
            (0x2a, 2, false, Accumulator),
            (0x26, 5, false, ZeroPage),
            (0x36, 6, false, ZeroPageX),
            (0x2e, 6, false, Absolute),
            (0x3e, 7, false, AbsoluteX),
        ],
    );

    add_group(
        ROR,
        op_rotate,
        &[
            (0x6a, 2, false, Accumulator),
            (0x66, 5, false, ZeroPage),
            (0x76, 6, false, ZeroPageX),
            (0x6e, 6, false, Absolute),
            (0x7e, 7, false, AbsoluteX),
        ],
    );

    add_group(
        SBC,
        op_arithmetic,
        &[
            (0xe9, 2, false, Immediate),
            (0xe5, 3, false, ZeroPage),
            (0xf5, 4, false, ZeroPageX),
            (0xed, 3, false, Absolute),
            (0xfd, 4, true, AbsoluteX),
            (0xf9, 4, true, AbsoluteY),
            (0xe1, 6, false, IndirectX),
            (0xf1, 5, true, IndirectY),
        ],
    );

    add_group(
        STA,
        op_store,
        &[
            (0x85, 3, false, ZeroPage),
            (0x95, 4, false, ZeroPageX),
            (0x8d, 4, false, Absolute),
            (0x9d, 5, false, AbsoluteX),
            (0x99, 5, false, AbsoluteY),
            (0x81, 6, false, IndirectX),
            (0x91, 6, false, IndirectY),
        ],
    );

    add_group(
        STX,
        op_store,
        &[
            (0x86, 3, false, ZeroPage),
            (0x96, 4, false, ZeroPageY),
            (0x8e, 4, false, Absolute),
        ],
    );

    add_group(
        STY,
        op_store,
        &[
            (0x84, 3, false, ZeroPage),
            (0x94, 4, false, ZeroPageX),
            (0x8c, 4, false, Absolute),
        ],
    );

    add_group(
        JMP,
        op_jmp,
        &[(0x4c, 3, false, Absolute), (0x6c, 5, false, Indirect)],
    );

    // branching
    add_functional_group(
        2,
        true,
        Relative,
        op_branch,
        &[
            (BCC, 0x90),
            (BCS, 0xb0),
            (BEQ, 0xf0),
            (BNE, 0xd0),
            (BPL, 0x10),
            (BMI, 0x30),
            (BVC, 0x50),
            (BVS, 0x70),
        ],
    );

    // flag set/unset
    add_functional_group(
        2,
        false,
        Implicit,
        op_flag,
        &[
            (CLC, 0x18),
            (SEC, 0x38),
            (CLD, 0xd8),
            (SED, 0xf8),
            (CLI, 0x58),
            (SEI, 0x78),
            (CLV, 0xb8),
        ],
    );

    // transfer (between registers)
    add_functional_group(
        2,
        false,
        Implicit,
        op_transfer,
        &[
            (TAX, 0xaa),
            (TAY, 0xa8),
            (TXA, 0x8a),
            (TYA, 0x98),
            (TXS, 0x9a),
            (TSX, 0xba),
        ],
    );

    // stack push
    add_functional_group(3, false, Implicit, op_push, &[(PHA, 0x48), (PHP, 0x08)]);
    //
    // incrementation / decrementation
    add_functional_group(
        2,
        false,
        Implicit,
        op_incdec_reg,
        &[(DEX, 0xca), (DEY, 0x88), (INX, 0xe8), (INY, 0xc8)],
    );

    // jumps and returns
    add_op(JSR, 0x20, 6, false, Absolute, op_jsr);
    add_op(RTS, 0x60, 6, false, Implicit, op_rts);

    // other
    add_op(BRK, 0x00, 7, false, Implicit, op_brk);
    add_op(PLA, 0x68, 4, false, Implicit, op_pla);
    add_op(PLP, 0x28, 4, false, Implicit, op_plp);
    add_op(RTI, 0x40, 6, false, Implicit, op_rti);

    for (key, val) in ops3.into_iter() {
        if o.contains_key(&key) {
            panic!("{} opcode already exists in the opsmap", key);
        }
        o.insert(key, val);
    }

    o
}

// ----------------------------------------------------------------------
// helpers

fn get_val(op: &Operation, machine: &Machine) -> Option<u8> {
    if let Some(addr) = op.address {
        Some(machine.mem.get_byte(addr))
    } else if op.def.address_mode == Immediate {
        op.operand.as_ref().unwrap().get_byte()
    } else if op.def.address_mode == Accumulator {
        Some(machine.A8())
    } else {
        None
    }
}

fn set_val(val: u8, op: &Operation, machine: &mut Machine) {
    if let Some(addr) = op.address {
        machine.mem.set_byte(addr, val)
    } else if op.def.address_mode == Accumulator {
        machine.set_A(val)
    } else {
        panic!("Can't set value for address mode {}", op.def.address_mode)
    };
}

fn store_byte(val: u8, op: &Operation, machine: &mut Machine) -> u8 {
    machine.mem.set_byte(op.address.unwrap(), val);
    op.def.cycles
}

fn set_flags(flags: &str, vals: &[bool], machine: &mut Machine) {
    let chars = String::from(flags);
    if chars.len() != vals.len() {
        panic!("Incorrect args length in set_flags")
    };
    for (i, ch) in chars.chars().enumerate() {
        match ch {
            'C' => machine.cpu.registers.status.carry = vals[i],
            'Z' => machine.cpu.registers.status.zero = vals[i],
            'I' => machine.cpu.registers.status.interrupt_disable = vals[i],
            'D' => machine.cpu.registers.status.decimal_mode = vals[i],
            'B' => machine.cpu.registers.status.break_command = vals[i],
            'V' => machine.cpu.registers.status.overflow = vals[i],
            'N' => machine.cpu.registers.status.negative = vals[i],
            _ => panic!("Unrecognized flag symbol: {}", ch),
        };
    }
}

fn set_nz_flags(val: u8, machine: &mut Machine) {
    machine.cpu.registers.status.negative = neg(val);
    machine.cpu.registers.status.zero = zero(val);
}

fn neg(val: u8) -> bool {
    val & 0x80 > 0
}
fn zero(val: u8) -> bool {
    val == 0
}

// see https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
fn overflow(in1: u8, in2: u8, result: u8) -> bool {
    ((in1 ^ result) & (in2 ^ result) & 0x80) > 0
}

// ----------------------------------------------------------------------
// implementation of operations

// https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
// http://retro.hansotten.nl/uploads/mag6502/sbc_tsx_txs_instructions.pdf
// TODO compute cycles for page cross
fn op_arithmetic(op: &Operation, machine: &mut Machine) -> u8 {
    let a = machine.A8();
    let val = match op.def.mnemonic {
        ADC => get_val(op, machine).unwrap(),
        SBC => !get_val(op, machine).unwrap(),
        _ => panic!("{} is not an arithmetic operation", op.def.mnemonic),
    };
    let sum = machine.A16() + u16::from(machine.P().carry) + val as u16;
    machine.set_A((sum & 0xff) as u8);
    let res = machine.A8();
    set_flags(
        "NZCV",
        &[neg(res), zero(res), sum > 0xff, overflow(a, val, res)],
        machine,
    );
    op.def.cycles
}

fn op_bit(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    set_flags(
        "NZV",
        &[neg(val), zero(val & machine.A8()), val & 0b01000000 > 0],
        machine,
    );
    op.def.cycles
}

fn op_branch(op: &Operation, machine: &mut Machine) -> u8 {
    let branch: bool = match op.def.mnemonic {
        BCC => !machine.P().carry,
        BCS => machine.P().carry,
        BNE => !machine.P().zero,
        BEQ => machine.P().zero,
        BPL => !machine.P().negative,
        BMI => machine.P().negative,
        BVC => !machine.P().overflow,
        BVS => machine.P().overflow,
        _ => panic!("{} is not a branch operation", op.def.mnemonic),
    };
    if branch {
        machine.cpu.registers.counter = op.address.unwrap();
        return op.def.cycles + 1; // TODO consider page change
    }
    op.def.cycles
}

fn op_brk(op: &Operation, machine: &mut Machine) -> u8 {
    set_flags("B", &[true], machine);
    op.def.cycles
}

// TODO add cycle for page change
fn op_compare(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    let reg = match op.def.mnemonic {
        CMP => machine.A8(),
        CPX => machine.X8(),
        CPY => machine.Y8(),
        _ => panic!("{} is not a compare operation", op.def.mnemonic),
    };
    let diff = (Wrapping(reg) - Wrapping(val)).0;
    set_flags("NZC", &[neg(diff), reg == val, reg >= val], machine);
    op.def.cycles
}

fn op_incdec_mem(op: &Operation, machine: &mut Machine) -> u8 {
    let mut val = Wrapping(get_val(op, machine).unwrap());
    match op.def.mnemonic {
        DEC => val -= 1,
        INC => val += 1,
        _ => panic!("{} is not a inc/dec (mem) operation", op.def.mnemonic),
    };
    set_val(val.0, op, machine);
    set_nz_flags(val.0, machine);
    op.def.cycles
}

fn op_incdec_reg(op: &Operation, machine: &mut Machine) -> u8 {
    match op.def.mnemonic {
        DEX => machine.cpu.registers.x -= 1,
        DEY => machine.cpu.registers.y -= 1,
        INX => machine.cpu.registers.x += 1,
        INY => machine.cpu.registers.y += 1,
        _ => panic!("{} is not a inc/dec operation", op.def.mnemonic),
    };
    let val = match op.def.mnemonic {
        DEX | INX => machine.X8(),
        DEY | INY => machine.Y8(),
        _ => panic!("{} is not a inc/dec operation", op.def.mnemonic),
    };
    set_nz_flags(val, machine);
    op.def.cycles
}

// TODO add cycle for page change
fn op_bitwise(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    match op.def.mnemonic {
        AND => machine.cpu.registers.accumulator &= val,
        ORA => machine.cpu.registers.accumulator |= val,
        EOR => machine.cpu.registers.accumulator ^= val,
        _ => panic!("{} is not a bitwise operation", op.def.mnemonic),
    };
    set_nz_flags(machine.A8(), machine);
    op.def.cycles
}

fn op_flag(op: &Operation, machine: &mut Machine) -> u8 {
    match op.def.mnemonic {
        CLC => machine.cpu.registers.status.carry = false,
        SEC => machine.cpu.registers.status.carry = true,
        CLI => machine.cpu.registers.status.interrupt_disable = false,
        SEI => machine.cpu.registers.status.interrupt_disable = true,
        CLD => machine.cpu.registers.status.decimal_mode = false,
        SED => machine.cpu.registers.status.decimal_mode = true,
        CLV => machine.cpu.registers.status.overflow = false,
        _ => panic!("{} is not a flag set/unset operation", op.def.mnemonic),
    };
    op.def.cycles
}

fn op_jmp(op: &Operation, machine: &mut Machine) -> u8 {
    machine.cpu.registers.counter = op.address.unwrap();
    op.def.cycles
}

fn op_jsr(op: &Operation, machine: &mut Machine) -> u8 {
    let pc = machine.PC().wrapping_sub(1);
    machine.push((pc >> 8) as u8);
    machine.push((pc & 0x00ff) as u8);
    machine.cpu.registers.counter = op.address.unwrap();
    op.def.cycles
}

// FIXME add cycle for crossing page boundary
fn op_load(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    match op.def.mnemonic {
        LDA => machine.set_A(val),
        LDX => machine.set_X(val),
        LDY => machine.set_Y(val),
        _ => panic!("{} is not a load operation", op.def.mnemonic),
    };
    set_nz_flags(val, machine);
    op.def.cycles
}

fn op_nop(op: &Operation, _machine: &mut Machine) -> u8 {
    op.def.cycles
}

fn op_pla(op: &Operation, machine: &mut Machine) -> u8 {
    let val = machine.pop();
    machine.set_A(val);
    set_nz_flags(val, machine);
    op.def.cycles
}

fn op_plp(op: &Operation, machine: &mut Machine) -> u8 {
    let val = machine.pop();
    machine.cpu.registers.status = ProcessorStatus::from(val);
    op.def.cycles
}

fn op_push(op: &Operation, machine: &mut Machine) -> u8 {
    let val = match op.def.mnemonic {
        PHA => machine.A8(),
        PHP => u8::from(&machine.P()),
        _ => panic!("{} is not a push operation", op.def.mnemonic),
    };
    machine.mem.set_byte(machine.stack_addr(), val);
    machine.cpu.registers.stack -= 1;
    op.def.cycles
}

fn op_rotate(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    let c = if machine.P().carry { 0xff } else { 0 };
    let (new_val, mask, carry) = match op.def.mnemonic {
        ROL => (val << 1, c & 1, val & 0b10000000 > 0),
        ROR => (val >> 1, c & 0b10000000, val & 1 > 0),
        _ => panic!("{} is not a rotate operation", op.def.mnemonic),
    };
    set_val(new_val | mask, op, machine);
    set_flags(
        "NZC",
        &[neg(new_val | mask), zero(new_val | mask), carry],
        machine,
    );
    op.def.cycles
}

fn op_rti(op: &Operation, machine: &mut Machine) -> u8 {
    machine.cpu.registers.status = ProcessorStatus::from(machine.pop());
    machine.cpu.registers.counter = machine.pop() as u16 | ((machine.pop() as u16) << 8);
    op.def.cycles
}

fn op_rts(op: &Operation, machine: &mut Machine) -> u8 {
    machine.cpu.registers.counter =
        (machine.pop() as u16 | ((machine.pop() as u16) << 8)).wrapping_add(1);
    op.def.cycles
}

fn op_shift(op: &Operation, machine: &mut Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    let (res, carry) = match op.def.mnemonic {
        ASL => ((Wrapping(val) << 1).0, val & 0b10000000 > 0),
        LSR => (val >> 1, val & 1 > 0),
        _ => panic!("{} is not a shift operation", op.def.mnemonic),
    };
    set_val(res, op, machine);
    set_flags("NZC", &[neg(res), zero(res), carry], machine);
    op.def.cycles
}

fn op_store(op: &Operation, machine: &mut Machine) -> u8 {
    match op.def.mnemonic {
        STA => store_byte(machine.A8(), op, machine),
        STX => store_byte(machine.X8(), op, machine),
        STY => store_byte(machine.Y8(), op, machine),
        _ => panic!("{} is not a store operation", op.def.mnemonic),
    }
}

fn op_transfer(op: &Operation, machine: &mut Machine) -> u8 {
    match op.def.mnemonic {
        TAX => machine.set_X(machine.A()),
        TAY => machine.set_Y(machine.A()),
        TXA => machine.set_A(machine.X()),
        TYA => machine.set_A(machine.Y()),
        TXS => machine.set_SC(machine.X()),
        TSX => machine.set_X(machine.SC()),
        _ => panic!("{} is not a transfer operation", op.def.mnemonic),
    };
    if op.def.mnemonic != TXS {
        // TXS doesn't change any flag
        set_nz_flags(machine.A8(), machine);
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
