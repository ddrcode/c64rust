use super::{AddressMode::*, Mnemonic::*, Operation, ProcessorStatus};
use crate::{
    machine::Machine,
    utils::{bcd_to_dec, dec_to_bcd},
};
use std::num::Wrapping;

// OMG this is so terribly ugly!
pub fn execute_operation<T: Machine>(op: &Operation, machine: &mut T) -> u8 {
    match &*op.def.fn_name {
        "op_arithmetic" => {
            if machine.cpu().registers.status.decimal_mode {
                op_arithmetic_bcd(op, machine)
            } else {
                op_arithmetic(op, machine)
            }
        }
        "op_bit" => op_bit(op, machine),
        "op_bitwise" => op_bitwise(op, machine),
        "op_branch" => op_branch(op, machine),
        "op_brk" => op_brk(op, machine),
        "op_compare" => op_compare(op, machine),
        "op_flag" => op_flag(op, machine),
        "op_incdec_mem" => op_incdec_mem(op, machine),
        "op_incdec_reg" => op_incdec_reg(op, machine),
        "op_jmp" => op_jmp(op, machine),
        "op_jsr" => op_jsr(op, machine),
        "op_load" => op_load(op, machine),
        "op_nop" => op_nop(op, machine),
        "op_pla" => op_pla(op, machine),
        "op_plp" => op_plp(op, machine),
        "op_push" => op_push(op, machine),
        "op_rotate" => op_rotate(op, machine),
        "op_rti" => op_rti(op, machine),
        "op_rts" => op_rts(op, machine),
        "op_shift" => op_shift(op, machine),
        "op_store" => op_store(op, machine),
        "op_transfer" => op_transfer(op, machine),
        _ => panic!("Unidentified function name {}", op.def.fn_name),
    }
}

// ----------------------------------------------------------------------
// helpers

fn get_val(op: &Operation, machine: &impl Machine) -> Option<u8> {
    if let Some(addr) = op.address {
        Some(machine.read_byte(addr))
    } else if op.def.address_mode == Immediate {
        op.operand.as_ref().unwrap().get_byte()
    } else if op.def.address_mode == Accumulator {
        Some(machine.A8())
    } else {
        None
    }
}

fn set_val(val: u8, op: &Operation, machine: &mut impl Machine) {
    if let Some(addr) = op.address {
        machine.write_byte(addr, val)
    } else if op.def.address_mode == Accumulator {
        machine.set_A(val)
    } else {
        panic!("Can't set value for address mode {}", op.def.address_mode)
    };
}

fn store_byte(val: u8, op: &Operation, machine: &mut impl Machine) -> u8 {
    machine.write_byte(op.address.unwrap(), val);
    op.def.cycles
}

fn set_flags(flags: &str, vals: &[bool], machine: &mut impl Machine) {
    let chars = String::from(flags);
    if chars.len() != vals.len() {
        panic!("Incorrect args length in set_flags")
    };
    for (i, ch) in chars.chars().enumerate() {
        match ch {
            'C' => machine.cpu_mut().registers.status.carry = vals[i],
            'Z' => machine.cpu_mut().registers.status.zero = vals[i],
            'I' => machine.cpu_mut().registers.status.interrupt_disable = vals[i],
            'D' => machine.cpu_mut().registers.status.decimal_mode = vals[i],
            'B' => machine.cpu_mut().registers.status.break_command = vals[i],
            'V' => machine.cpu_mut().registers.status.overflow = vals[i],
            'N' => machine.cpu_mut().registers.status.negative = vals[i],
            _ => panic!("Unrecognized flag symbol: {}", ch),
        };
    }
}

fn set_nz_flags(val: u8, machine: &mut impl Machine) {
    machine.cpu_mut().registers.status.negative = neg(val);
    machine.cpu_mut().registers.status.zero = zero(val);
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
fn op_arithmetic(op: &Operation, machine: &mut impl Machine) -> u8 {
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

// see http://www.6502.org/tutorials/decimal_mode.html
fn op_arithmetic_bcd(op: &Operation, machine: &mut impl Machine) -> u8 {
    let a = bcd_to_dec(machine.A8());
    let val = bcd_to_dec(get_val(op, machine).unwrap());
    let (sum, carry) = match op.def.mnemonic {
        ADC => {
            let x = a + u8::from(machine.P().carry) + val;
            (x, x > 99)
        }
        SBC => {
            let x = a
                .wrapping_sub(u8::from(!machine.P().carry))
                .wrapping_sub(val);
            if x >= 156 {
                (x - 156, false)
            } else {
                (x, true)
            }
        }
        _ => panic!("{} is not an arithmetic operation", op.def.mnemonic),
    };
    let res = sum - 100 * (sum / 100);
    machine.set_A(dec_to_bcd(res));
    set_flags(
        "NZCV",
        &[neg(res), zero(res), carry, overflow(a, val, res)],
        machine,
    );
    op.def.cycles
}

fn op_bit(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    set_flags(
        "NZV",
        &[neg(val), zero(val & machine.A8()), val & 0b01000000 > 0],
        machine,
    );
    op.def.cycles
}

fn op_branch(op: &Operation, machine: &mut impl Machine) -> u8 {
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
        machine.set_PC(op.address.unwrap());
        return op.def.cycles + 1; // TODO consider page change
    }

    // BVC always takes 3 cycles (see https://c64os.com/post/6502instructions)
    op.def.cycles + if op.def.mnemonic == BVC { 1 } else { 0 }
}

// see https://www.c64-wiki.com/wiki/BRK
fn op_brk(op: &Operation, machine: &mut impl Machine) -> u8 {
    machine.set_PC(machine.PC().wrapping_add(2));
    set_flags("B", &[true], machine);
    machine.irq();
    op.def.cycles
}

// TODO add cycle for page change
fn op_compare(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    let reg = match op.def.mnemonic {
        CMP => machine.A8(),
        CPX => machine.X8(),
        CPY => machine.Y8(),
        _ => panic!("{} is not a compare operation", op.def.mnemonic),
    };
    let diff = (Wrapping(reg) - Wrapping(val)).0;
    set_flags("NZC", &[neg(diff), reg == val, val <= reg], machine);
    op.def.cycles
}

fn op_incdec_mem(op: &Operation, machine: &mut impl Machine) -> u8 {
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

fn op_incdec_reg(op: &Operation, machine: &mut impl Machine) -> u8 {
    match op.def.mnemonic {
        DEX => machine.cpu_mut().registers.x -= 1,
        DEY => machine.cpu_mut().registers.y -= 1,
        INX => machine.cpu_mut().registers.x += 1,
        INY => machine.cpu_mut().registers.y += 1,
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
fn op_bitwise(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val = get_val(op, machine).unwrap();
    match op.def.mnemonic {
        AND => machine.cpu_mut().registers.accumulator &= val,
        ORA => machine.cpu_mut().registers.accumulator |= val,
        EOR => machine.cpu_mut().registers.accumulator ^= val,
        _ => panic!("{} is not a bitwise operation", op.def.mnemonic),
    };
    set_nz_flags(machine.A8(), machine);
    op.def.cycles
}

fn op_flag(op: &Operation, machine: &mut impl Machine) -> u8 {
    match op.def.mnemonic {
        CLC => machine.cpu_mut().registers.status.carry = false,
        SEC => machine.cpu_mut().registers.status.carry = true,
        CLI => machine.cpu_mut().registers.status.interrupt_disable = false,
        SEI => machine.cpu_mut().registers.status.interrupt_disable = true,
        CLD => machine.cpu_mut().registers.status.decimal_mode = false,
        SED => machine.cpu_mut().registers.status.decimal_mode = true,
        CLV => machine.cpu_mut().registers.status.overflow = false,
        _ => panic!("{} is not a flag set/unset operation", op.def.mnemonic),
    };
    op.def.cycles
}

fn op_jmp(op: &Operation, machine: &mut impl Machine) -> u8 {
    machine.set_PC(op.address.unwrap());
    op.def.cycles
}

fn op_jsr(op: &Operation, machine: &mut impl Machine) -> u8 {
    let pc = machine.PC().wrapping_sub(1);
    machine.push((pc >> 8) as u8);
    machine.push((pc & 0x00ff) as u8);
    machine.set_PC(op.address.unwrap());
    op.def.cycles
}

// FIXME add cycle for crossing page boundary
fn op_load(op: &Operation, machine: &mut impl Machine) -> u8 {
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

fn op_nop(op: &Operation, _machine: &mut impl Machine) -> u8 {
    op.def.cycles
}

fn op_pla(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val = machine.pop();
    machine.set_A(val);
    set_nz_flags(val, machine);
    op.def.cycles
}

fn op_plp(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val = machine.pop();
    machine.cpu_mut().registers.status = ProcessorStatus::from(val);
    op.def.cycles
}

fn op_push(op: &Operation, machine: &mut impl Machine) -> u8 {
    let val: u8 = match op.def.mnemonic {
        PHA => machine.A8(),
        PHP => machine.P().into(),
        _ => panic!("{} is not a push operation", op.def.mnemonic),
    };
    let addr = machine.stack_addr();
    machine.write_byte(addr, val);
    machine.cpu_mut().registers.stack -= 1;
    op.def.cycles
}

fn op_rotate(op: &Operation, machine: &mut impl Machine) -> u8 {
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

fn op_rti(op: &Operation, machine: &mut impl Machine) -> u8 {
    machine.cpu_mut().registers.status = ProcessorStatus::from(machine.pop());
    machine.cpu_mut().registers.counter = machine.pop() as u16 | ((machine.pop() as u16) << 8);
    op.def.cycles
}

fn op_rts(op: &Operation, machine: &mut impl Machine) -> u8 {
    let lo = machine.pop() as u16;
    let hi = machine.pop() as u16;
    machine.set_PC((lo | hi << 8).wrapping_add(1));
    op.def.cycles
}

fn op_shift(op: &Operation, machine: &mut impl Machine) -> u8 {
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

fn op_store(op: &Operation, machine: &mut impl Machine) -> u8 {
    match op.def.mnemonic {
        STA => store_byte(machine.A8(), op, machine),
        STX => store_byte(machine.X8(), op, machine),
        STY => store_byte(machine.Y8(), op, machine),
        _ => panic!("{} is not a store operation", op.def.mnemonic),
    }
}

fn op_transfer(op: &Operation, machine: &mut impl Machine) -> u8 {
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

    #[test]
    fn test_bcd_conversions() {
        assert_eq!(45, bcd_to_dec(0x45));
    }
}
