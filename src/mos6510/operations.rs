use super::{ Operation, Mnemonic, AddressMode, OpFn, OpsMap, Operand };
use crate::c64::C64;

pub fn define_operations(o: &mut OpsMap) -> &OpsMap {
    type OpData = (u8, u8, bool, AddressMode);
    type AM = AddressMode;
    type M = Mnemonic;

    let mut add_op = |mnemonic: Mnemonic, opcode: u8, cycles: u8, boundary: bool, am: AddressMode, opfn: OpFn| {
        o.insert(opcode, Operation {
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
    
    add_group(M::ADC, op_adc, &[
        (0x69, 2, false, AM::Immediate),
        (0x65, 3, false, AM::ZeroPage)
    ]);


    add_op(M::BRK, 0x00, 7, false, AM::Implicit, op_brk);
    
    o
}

fn op_adc(op: &Operation, operand: &Operand, c64: &mut C64) {
    let val = match op.address_mode {
        AddressMode::Immediate => operand.get_byte().unwrap(),
        _ => panic!("Unsupported address mode: {}", op.address_mode)
    };
    println!("Accumulator is {}, value is {}", c64.cpu.registers.counter, val);
    c64.cpu.registers.accumulator += val;
}


fn op_brk(_op: &Operation, _data: &Operand, c64: &mut C64) {}
