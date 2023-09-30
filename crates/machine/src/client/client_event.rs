use std::ops::Range;

use keyboard_types::KeyboardEvent;

use crate::debugger::Breakpoint;

#[derive(Debug, PartialEq)]
pub enum ClientEvent {
    EnableBreakpoint(Breakpoint),
    DisableBreakpoint(Breakpoint),
    KeyPress(KeyboardEvent),
    SetObservedMemory(Range<u16>)
}
