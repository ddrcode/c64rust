use super::{
    Operation, OperationDef, Mnemonic, Mnemonic::*, AddressMode, AddressMode::*, OpFn, OpsMap, Operand,
    ProcessorStatus
};
use crate::c64::C64;

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

    add_group(CMP, op_cmp, &[
        (0xc9, 2, false, Immediate),
        (0xc5, 3, false, ZeroPage),
        (0xd5, 4, false, ZeroPageX),
        (0xcd, 4, false, Absolute),
    ]);

    add_group(EOR, op_eor, &[
        (0x49, 2, false, Immediate),
        (0x45, 3, false, ZeroPage),
        (0x55, 4, false, ZeroPageX),
        (0x4d, 4, false, Absolute),
    ]);

    add_group(LDA, op_load, &[
        (0xa9, 2, false, Immediate),
        (0xa5, 3, false, ZeroPage),
        (0xb5, 4, false, ZeroPageX),
        (0xad, 3, false, Absolute),
        (0xa1, 2, false, IndirectX),
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
        &[(TAX, 0xaa), (TAY, 0xa8), (TXA, 0x8a), (TYA, 0x98)]);

    // stack push
    add_functional_group(3, false, Implicit, op_push,
        &[(PHA, 0x48), (PHP, 0x08)]);


    // jumps and returns
    add_op(JSR, 0x20, 6, false, Absolute, op_jsr);
    add_op(RTS, 0x60, 6, false, Implicit, op_rts);

    // other
    add_op(BRK, 0x00, 7, false, Implicit, op_brk);
    add_op(DEX, 0xca, 2, false, Implicit, op_dex);

    o.extend(ops3);
    o
}

// helpers

fn get_val(op: &Operation, c64: &C64) -> Option<u8> {
    if let Some(addr) = op.address { Some(c64.mem.get_byte(addr)) }
    else if op.def.address_mode == Immediate { op.operand.as_ref().unwrap().get_byte() }
    else if op.def.address_mode == Accumulator { Some(c64.A()) }
    else { None }
}

fn set_val(val: u8, op: &Operation, c64: &mut C64) {
    if let Some(addr) = op.address { c64.mem.set_byte(addr, val) }
    else if op.def.address_mode == Accumulator { c64.cpu.registers.accumulator = val }
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

// implementation of operations

fn op_adc(op: &Operation, c64: &mut C64) -> u8 {
    c64.cpu.registers.accumulator += get_val(op, c64).unwrap();
    set_flags("NZCV", &[false, c64.A()==0, false, false], c64); // TODO fix C and V
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

fn op_cmp(op: &Operation, c64: &mut C64) -> u8 {
    let a = c64.A();
    let val = get_val(op, c64).unwrap();
    // TODO fix N flag: it should be a sign bit of the result
    // TODO check C flag - whether > or >= operator should be used
    set_flags("NZC", &[false, a==val, a > val], c64);
    op.def.cycles
}

fn op_dex(op: &Operation, c64: &mut C64) -> u8 {
    c64.cpu.registers.x -= 1;
    set_flags("NZ", &[false, c64.X() == 0], c64); // TODO N flag?
    op.def.cycles
}

fn op_eor(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    c64.cpu.registers.accumulator ^= val;
    set_flags("NZ", &[false, c64.X() == 0], c64); // TODO N flag?
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

fn op_load(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    match op.def.mnemonic {
        LDA => c64.cpu.registers.accumulator = val,
        LDX => c64.cpu.registers.x = val,
        LDY => c64.cpu.registers.y = val,
        _ => panic!("{} is not a load operation", op.def.mnemonic)
    };
    set_flags("NZ", &[false, val==0], c64); // TODO N flag?
    op.def.cycles
}

fn op_nop(op: &Operation, c64: &mut C64) -> u8 {
    op.def.cycles
}

fn op_push(op: &Operation, c64: &mut C64) -> u8 {
    let addr = 0x0100 | c64.SC() as u16;
    let val = match op.def.mnemonic {
        PHA => c64.A(),
        PHP => u8::from(&c64.P()),
        _ => panic!("{} is not a push operation", op.def.mnemonic)
    };
    c64.mem.set_byte(addr, val);
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

fn op_sbc(op: &Operation, c64: &mut C64) -> u8 {
    let val = get_val(op, c64).unwrap();
    c64.cpu.registers.accumulator -= val;
    let a = c64.A();
    set_flags("NZCV", &[a<val, a==0, a<val, false], c64); // TODO N and V flag?
    op.def.cycles
}

fn op_store(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        STA => store_byte(c64.A(), op, c64),
        STX => store_byte(c64.X(), op, c64),
        STY => store_byte(c64.Y(), op, c64),
        _ => panic!("{} is not a store operation", op.def.mnemonic)
    }
}

fn op_transfer(op: &Operation, c64: &mut C64) -> u8 {
    match op.def.mnemonic {
        TAX => c64.cpu.registers.x = c64.A(),
        TAY => c64.cpu.registers.y = c64.A(),
        TXA => c64.cpu.registers.accumulator = c64.X(),
        TYA => c64.cpu.registers.accumulator = c64.Y(),
        _ => panic!("{} is not a transfer operation", op.def.mnemonic)
    };
    set_flags("NZ", &[false, c64.A()==0], c64); // TODO N flag?
    op.def.cycles
}

