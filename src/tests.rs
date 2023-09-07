#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod arithmetic_tests; // ADC, SBC

#[cfg(test)]
mod comparison_tests; // CMP

#[cfg(test)]
mod jump_tests; //

#[cfg(test)]
mod branch_tests; // BPL

#[cfg(test)]
mod shift_tests;

pub use test_utils::asm_test;
