# c64rust

This is a simple (and incomplete) C64 emulator implemented in Rust. The main purpose of this project is to
practice my Rust skills and turn into reality my old dream of building an emulator.

## How to run

1. Download ROM file
2. Execute:
   `cargo run -- --rom your-rom-file -s -d --stop-on-addr e5d1`

The instruction above boots the rom and breaks on infinite loop waiting for
user input. Then it prints screen memory.

Running options:

```
Usage: c64emu [OPTIONS]

Options:
  -r, --rom <ROM>
      --ram <RAM>
      --ram-file-addr <RAM_FILE_ADDR>  [default: 0]
  -a, --start-addr <START_ADDR>        [default: fce2]
  -s, --show-screen
  -d, --disassemble
      --max-cycles <MAX_CYCLES>
      --max-time <MAX_TIME>
      --stop-on-addr <STOP_ON_ADDR>
  -h, --help                           Print help
  -V, --version                        Print version
```

## Current state

1. Download ROM file
2. Execute:
   `cargo run -- --rom your-rom-file -s -d --stop-on-addr e5d1`

The instruction above boots the rom and breaks on infinite loop waiting for
user input. Then it prints screen memory.

Running options:

```
Usage: c64emu [OPTIONS]

Options:
  -r, --rom <ROM>
      --ram <RAM>
      --ram-file-addr <RAM_FILE_ADDR>  [default: 0]
  -a, --start-addr <START_ADDR>        [default: fce2]
  -s, --show-screen
  -d, --disassemble
      --max-cycles <MAX_CYCLES>
      --max-time <MAX_TIME>
      --stop-on-addr <STOP_ON_ADDR>
  -h, --help                           Print help
  -V, --version                        Print version
```

## Current state
- MOS6510 (6510) instruction set fully implemented
- C64 memory addressing implemented (RAM/ROM switching)
- The emulator can print out current screen memory (text only)
- The emulator boots (with some errors, but doesn't break) with provided C64 ROM

This is the result of running current version of the emulator:
![Screenshot](screenshots/first-version-with-bugs.png?raw=true "First (almost) working version")

## Features and goals

### Short-term / realistic goals

- Booting with errors means (most likely) 6502 emulation contains bugs; the primary goal is to find
  and fix them (tests being actively developed)
- Ability to boot the emulator with binary kernel and successfully load it
- Clock emulation (currently it ticks at host speed; it's not an issue as there is no emulation of
  other devices - means no IRQs).

### Ambitions

- Keyboard emulation
- VIC II graphics (without sprites and smooth scrolling)

### Long-term goals

- Sprites and smooth scrolling
- Sound
- Joystick emulation
- Cartridge binaries support

### No-goals

- Making competitive product to Vice and other well-established emulators.
- CRT emulation

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

#### Signed/unsigned numbers

- [More about binary numbers](http://www.emulator101.com/more-about-binary-numbers.html)

### Memory

#### Memory map

- [Commodore 64 memory map](https://sta.c64.org/cbm64mem.html)
- [C64 Memory Map](https://www.pagetable.com/c64ref/c64mem/)

#### Bank switching

- [Bank switching on C64 wiki](https://www.c64-wiki.com/wiki/Bank_Switching)

### Reset / Boot sequence

- [Reset (process) on C64 Wiki](https://www.c64-wiki.com/wiki/Reset_%28Process%29)
- [Internals of BRK/IRQ/NMI/RESET on a MOS 6502 by Michael Steil](https://www.pagetable.com/?p=410)

### Page boundaries

- [Page Boundaries](http://forum.6502.org/viewtopic.php?t=469)

### VIC II
