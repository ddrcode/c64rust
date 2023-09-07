# c64rust

_USE DEV BRANCH!!!_

This is a simple (and incomplete) C64 emulator implemented in Rust. The main purpose of this project is to 
practice my Rust skills and turn into reality my old dream of building an emulator.

## Current state

The emulator is in an early state of development. My current focus is to make MOS6502 instruction set 
implemented together with correct C64 RAM and ROM emulation. 

## Features and goals

### Short-term / realistic goals
- Complete MOS6510 emulation (with RAM and ROM addressing)
- Partial VIC II emulation (text mode only)
- Ability to boot the emulator with binary kernel and succesfully load it

### Ambitions
- Keyboard emulation
- VIC II graphics

### Long-term goals
- Sound
- Joystic simulation
- Cartridge binaries support

### No-goals
- Making competitive product to Vice and other well-estiablished emulators.

## References

Below there is a list of posts that helped me to gain actual knowledge about C64/MOS6510 architecture 
and answered many of my questions. 

### CPU
#### Instruction set
#### Memory access modes
- [Writing your own NES emulator Part 3 - the 6502 CPU](https://yizhang82.dev/nes-emu-cpu)
#### Status register
- [The 6502 Status Register: a Guide to Black Magic at 6502.org](http://forum.6502.org/viewtopic.php?f=2&t=6099)
#### BCD (Binary code decimal)
- [Decimal Mode by Bruce Clark (at 6502.org)](http://6502.org/tutorials/decimal_mode.html)

### Memory
#### Memory map
#### Bank switching
- [Bank switching on C64 wiki](https://www.c64-wiki.com/wiki/Bank_Switching)

### Reset / Boot sequence
- [Reset (process) on C64 Wiki](https://www.c64-wiki.com/wiki/Reset_%28Process%29)
- [Internals of BRK/IRQ/NMI/RESET on a MOS 6502 by Michael Steil](https://www.pagetable.com/?p=410)

### VIC II
