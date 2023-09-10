# c64rust

This is a simple (and incomplete) C64 emulator implemented in Rust. The main purpose of this project is to
practice my Rust skills and turn into reality my old dream of building an emulator.

## How to run

1. Download ROM file
2. Execute:
   `cargo run -- --rom your-rom-file -s -d --stop-on-addr e5d1`

The instruction above boots the rom and stops on an infinite loop waiting for
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

- MOS6502 (6510) instruction set fully implemented
- C64 memory addressing implemented (RAM/ROM switching)
- The emulator can print out current screen memory (text only)
- The emulator boots (with some errors, but doesn't break) with provided C64 ROM

This is the result of running current version of the emulator:

<img src="screenshots/first-version-with-bugs.png?raw=true" width="300"/>

## Features and goals

### Short-term / realistic goals

- Booting with errors means (most likely) 6502 emulation contains bugs; the primary goal is to find
  and fix them (tests being actively developed)
- Ability to boot the emulator with binary kernel and successfully load it
- Clock emulation (currently it ticks at host speed; it's not an issue as there is no emulation of
  other devices like GPU/VIC II, so sync is not required).

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

## Credits

- [srounce](https://github.com/srounce) - making the environment work with Nix Flake. So cool!

## References

Below there is a list of posts that helped me to gain actual knowledge about C64/MOS6510 architecture
and answered many of my questions.

### CPU

- [The 6502 Architecture (by prof William T. Verts)](https://people.cs.umass.edu/~verts/cmpsci201/spr_2004/Lecture_02_2004-01-30_The_6502_processor.pdf)

#### Instruction set

- [6502 / 6510 Instruction Set](https://c64os.com/post/6502instructions)
- [6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html#LSR)

#### Memory access modes

- [Writing your own NES emulator Part 3 - the 6502 CPU](https://yizhang82.dev/nes-emu-cpu)

#### Status register

- [The 6502 Status Register: a Guide to Black Magic at 6502.org](http://forum.6502.org/viewtopic.php?f=2&t=6099)
- [The 6502 overflow flag explained mathematically](https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html)
- [The Overflow (V) Flag Explained by Bruce Clark](http://www.6502.org/tutorials/vflag.html)

#### BCD (Binary code decimal)

- [Decimal Mode by Bruce Clark (at 6502.org)](http://6502.org/tutorials/decimal_mode.html)

#### Signed/unsigned numbers

- [More about binary numbers](http://www.emulator101.com/more-about-binary-numbers.html)
- [Beyond 8-bit Unsigned Comparisons by Bruce Clark](http://www.6502.org/tutorials/compare_beyond.html)

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

### VIC II and graphics

- [Double IRQ Explained](https://codebase64.org/doku.php?id=base:double_irq_explained)

### Keyboard

- [How the C64 Keyboard Works](https://www.c64os.com/post/howthekeyboardworks)

### Online emulators (for testing instructions behaviour)

- [Easy 6502](https://skilldrick.github.io/easy6502/)
- [Visual 6502](http://visual6502.org/JSSim/expert.html)
