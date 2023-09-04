// source: http://www.6502.org/users/obelisk/6502/registers.html
// source: https://www.nesdev.org/wiki/Status_flags
#[derive(Copy, Clone)]
pub struct ProcessorStatus {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow: bool,
    negative: bool
}

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false
        }
    }
}

impl From<&ProcessorStatus> for u8 {
    fn from(status: &ProcessorStatus) -> Self {
        status.carry as u8
        | bool_to_bit(&status.zero, 1)
        | (status.interrupt_disable as u8) << 2 
        | (status.decimal_mode as u8) << 3 
        | (status.break_command as u8) << 4 
        | 1 << 5 
        | (status.overflow as u8) << 6 
        | (status.negative as u8) << 7 
    }
}

fn bool_to_bit(val: &bool, bit: u8) -> u8 {
    if bit > 7 {
        panic!("illegal shift: shifting value by {} bits", bit); 
    }
    (*val as u8) << bit
}

fn bit_to_bool(val: &u8, bit: u8) -> bool {
    if bit > 7 {
        panic!("illegal shift: shifting value by {} bits", bit); 
    }
    (*val & (1 << bit)) > 0
}

impl From<u8> for ProcessorStatus {
    fn from(status: u8) -> Self {
        ProcessorStatus {
            carry: bit_to_bool(&status, 0),
            zero: bit_to_bool(&status, 1),
            interrupt_disable: bit_to_bool(&status, 2),
            decimal_mode: bit_to_bool(&status, 3),
            break_command: bit_to_bool(&status, 4),
            overflow: bit_to_bool(&status, 6),
            negative: bit_to_bool(&status, 7),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_into_u8() {
        let mut status = ProcessorStatus::new();
        assert_eq!(0b00100000, u8::from(&status));

        status.carry = true;
        assert_eq!(0b00100001, u8::from(&status));

        status.overflow = true;
        assert_eq!(0b01100001, u8::from(&status));
    }

    #[test]
    fn test_status_from_u8() {
        let val = | x: u8 | -> u8 { u8::from(&ProcessorStatus::from(x)) };
        let assert = | x: u8 | assert_eq!(x, val(x)); 
        assert(0b00100000);
        assert(0b00100001);
        assert(0b00100010);
        assert(0b00100100);
        assert(0b00101000);
        assert(0b00110000);
        assert(0b01100000);
        assert(0b10100000);
    }
}


