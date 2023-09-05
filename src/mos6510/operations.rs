use super::{
    Operation, OperationDef, Mnemonic, Mnemonic::*, AddressMode, AddressMode::*, OpFn, OpsMap, Operand
};
use crate::c64::C64;

pub fn define_operations(o: &mut OpsMap) -> &OpsMap {
    type OpData = (u8, u8, bool, AddressMode);

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

    add_group(ADC, op_adc, &[
        (0x69, 2, false, Immediate),
        (0x65, 3, false, ZeroPage)
    ]);


    add_op(BRK, 0x00, 7, false, Implicit, op_brk);

    o
}

fn op_adc(op: &Operation, c64: &mut C64) {
    let val = match op.def.address_mode {
        AddressMode::Immediate => op.operand.as_ref().unwrap().get_byte().unwrap(),
        _ => panic!("Unsupported address mode: {}", op.def.address_mode)
    };
    c64.cpu.registers.accumulator += val;
}


fn op_brk(_op: &Operation, c64: &mut C64) {}
