use thiserror::Error;
use crate::emulator::abstractions::Addr;

#[derive(Error, Debug)]
pub enum EmulatorError {

    #[error("Memory device can't read from address {0}")]
    AddressNotAccessible(Addr),


    #[error("Memory device can't write to address {0}")]
    AddressNotWriteable(Addr)

}


