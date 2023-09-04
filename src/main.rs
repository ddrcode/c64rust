mod mos6510;
mod c64;
mod machine;

use crate::c64::C64;

fn main() {
    let mut c64 = C64::new();
    c64.power_on();
    println!("C64 EMU. {} operations implemented", c64.cpu.operations.len());

    c64.load(&[0x69, 0x05, 0x69, 0x07, 0x00], 0x0100);
    c64.run(0x0100);

    println!("Accumulator: {}", c64.cpu.registers.accumulator);
}
