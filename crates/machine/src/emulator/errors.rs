use crate::emulator::abstractions::Addr;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum EmulatorError {
    #[error("Memory device can't read from address {0}")]
    AddressNotAccessible(Addr),

    #[error("Memory device can't write to address {0}")]
    AddressNotWriteable(Addr),

    #[error("Pin is already linked withanother link")]
    PinAlreadyLinked,

    #[error("Can't link ports of different widths")]
    IncompatiblePortWidths,

    #[error("Handler aleady defined for this port")]
    HandlerAlreadyDefined,

    #[error("Only tri-state pins can be enabled/disabled")]
    NotATriStatePin,

    #[error("Couldn't find Pin {1} in component {0}")]
    PinNotFound(String, String),

    #[error("Couldn't find component {0}")]
    ComponentNotFound(String)
}
