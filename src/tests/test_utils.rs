use crate::c64::*;

pub fn asm_test(prog: &[u8], res: u8, flags: u8) {
    let mut c64 = C64::new();
    c64.cpu.registers.counter = 0x0300;
    c64.load(prog, 0x0300);
    while c64.next() {}
    assert_eq!(res, c64.A8(), "Expecting result to be {:2x}", res);
    let res_flags = u8::from(&c64.P());
    assert_eq!(
        flags, res_flags,
        "Expecting flags to be {:b} but found {:b}",
        flags, res_flags
    );
}
