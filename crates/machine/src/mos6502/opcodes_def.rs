use super::{AddressMode, AddressMode::*, Mnemonic, Mnemonic::*, OperationDef, OpsMap};

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
                      opfn: &str| {
        if o.contains_key(&opcode) {
            panic!("Opcode {} already exists in the opsmap", opcode);
        }
        o.insert(
            opcode,
            OperationDef {
                opcode,
                mnemonic,
                cycles,
                page_boundary_cycle: boundary,
                address_mode: am,
                fn_name: opfn.to_string(),
            },
        );
    };

    let mut add_group = |mnemonic: Mnemonic, opfn: &str, data: &[OpData]| {
        for op in data.iter() {
            let (opcode, cycles, boundary, am) = *op;
            add_op(mnemonic, opcode, cycles, boundary, am, opfn);
        }
    };

    let mut add_functional_group =
        |cycles: u8, boundary: bool, am: AddressMode, opfn: &str, data: &[(Mnemonic, u8)]| {
            for op in data.iter() {
                let (mnemonic, opcode) = *op;
                ops3.insert(
                    opcode,
                    OperationDef {
                        opcode,
                        mnemonic,
                        cycles,
                        page_boundary_cycle: boundary,
                        address_mode: am,
                        fn_name: opfn.to_string(),
                    },
                );
            }
        };

    add_group(
        ADC,
        "op_arithmetic",
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
        "op_bitwise",
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
        "op_shift",
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
        "op_bit",
        &[(0x24, 3, false, ZeroPage), (0x2c, 4, false, Absolute)],
    );

    add_group(
        CMP,
        "op_compare",
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
        "op_compare",
        &[
            (0xe0, 2, false, Immediate),
            (0xe4, 3, false, ZeroPage),
            (0xec, 4, false, Absolute),
        ],
    );

    add_group(
        CPY,
        "op_compare",
        &[
            (0xc0, 2, false, Immediate),
            (0xc4, 3, false, ZeroPage),
            (0xcc, 4, false, Absolute),
        ],
    );

    add_group(
        DEC,
        "op_incdec_mem",
        &[
            (0xc6, 5, false, ZeroPage),
            (0xd6, 6, false, ZeroPageX),
            (0xce, 6, false, Absolute),
            (0xde, 7, false, AbsoluteX),
        ],
    );

    add_group(
        EOR,
        "op_bitwise",
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
        "op_incdec_mem",
        &[
            (0xe6, 5, false, ZeroPage),
            (0xf6, 6, false, ZeroPageX),
            (0xee, 6, false, Absolute),
            (0xfe, 7, false, AbsoluteX),
        ],
    );

    add_group(
        LDA,
        "op_load",
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
        "op_load",
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
        "op_load",
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
        "op_shift",
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
        "op_nop",
        &[
            (0xea, 2, false, Implicit),
            (0x80, 2, false, Immediate), // illegal
            (0x04, 3, false, ZeroPage),  // illegal
            (0x34, 4, false, ZeroPageX), // illegal
        ],
    );

    add_group(
        ORA,
        "op_bitwise",
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
        "op_rotate",
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
        "op_rotate",
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
        "op_arithmetic",
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
        "op_store",
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
        "op_store",
        &[
            (0x86, 3, false, ZeroPage),
            (0x96, 4, false, ZeroPageY),
            (0x8e, 4, false, Absolute),
        ],
    );

    add_group(
        STY,
        "op_store",
        &[
            (0x84, 3, false, ZeroPage),
            (0x94, 4, false, ZeroPageX),
            (0x8c, 4, false, Absolute),
        ],
    );

    add_group(
        JMP,
        "op_jmp",
        &[(0x4c, 3, false, Absolute), (0x6c, 5, false, Indirect)],
    );

    // branching
    add_functional_group(
        2,
        true,
        Relative,
        "op_branch",
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
        "op_flag",
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
        "op_transfer",
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
    add_functional_group(3, false, Implicit, "op_push", &[(PHA, 0x48), (PHP, 0x08)]);
    //
    // incrementation / decrementation
    add_functional_group(
        2,
        false,
        Implicit,
        "op_incdec_reg",
        &[(DEX, 0xca), (DEY, 0x88), (INX, 0xe8), (INY, 0xc8)],
    );

    // jumps and returns
    add_op(JSR, 0x20, 6, false, Absolute, "op_jsr");
    add_op(RTS, 0x60, 6, false, Implicit, "op_rts");

    // other
    add_op(BRK, 0x00, 7, false, Implicit, "op_brk");
    add_op(PLA, 0x68, 4, false, Implicit, "op_pla");
    add_op(PLP, 0x28, 4, false, Implicit, "op_plp");
    add_op(RTI, 0x40, 6, false, Implicit, "op_rti");

    for (key, val) in ops3.into_iter() {
        if o.contains_key(&key) {
            panic!("{} opcode already exists in the opsmap", key);
        }
        o.insert(key, val);
    }

    o
}
