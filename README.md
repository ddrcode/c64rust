# c64rust

This is a simple (and incomplete) CMOS6502 and C64 emulator/debugger implemented in Rust.
The main purpose of this project is to
practice my Rust skills and turn into reality my old dream of building an emulator.

When built, the project provides three binaries (in `target/debug`)

- `machine`: a generic 6502 emulator (CPU and memory only) for CLI,
- `c64`: CLI-based C64 emulator built on top of the above - good for testing: it
  can execute a binary and exit (on BRK or specific address) with processor status
  (or full disassembly)
- `gui`: ncurses-based client that allows for interaction with the emulator

## How to run

1. Download ROM file
2. Execute:
   `cargo run --bin gui  -- --rom path-to-rom`

If you don't have any ROM file, don't worry - I've created a test ROM from scratch!
It doesn't do much, but at least it displays a welcome message and a cursor.
You are welcome to contribute and help to make it an alternative C64 OS :-)
(the source is [here](https://github.com/ddrcode/c64rust/tree/main/rom))
But for now, just hit:
`cargo run --bin gui -- --rom rom/hello.rom`

### Running options:

```
Options:
  -r, --rom <ROM>
      --ram <RAM>
      --ram-file-addr <RAM_FILE_ADDR>  [default: 0200]
      --ram-size <RAM_SIZE>            [default: 65536]
  -a, --start-addr <START_ADDR>        [default: fce2]
  -s, --show-screen
      --show-status
  -d, --disassemble
      --max-cycles <MAX_CYCLES>
      --max-time <MAX_TIME>
      --stop-on-addr <STOP_ON_ADDR>
      --stop-on-brk
  -v, --verbose
  -h, --help                           Print help
  -V, --version                        Print version
```

## Current state

- MOS6502 (6510) instruction set fully implemented (no illegal opcodes)
- C64 memory addressing implemented (RAM/ROM switching, with partial CIA)
- The emulator boots with provided C64 ROM (some cartridges work too)
- Text client with keyboard emulation - possible to run BASIC commands
- The client has integrated simple debugging features: memory view, disassembler, and processor state
- Step-by-step debugging

This is the result of running current version of the emulator:

<img src="screenshots/disassembler.png?raw=true" width="800"/>

## Features and goals

### Short-term / realistic goals

- Clock emulation (right now the emu ticks at host speed; it's not an issue as there is no emulation of
  other devices like GPU/VIC II, so sync is not required).
- Basic CIA features (other than the keyboard)

### Ambitions

- VIC II graphics (without sprites and smooth scrolling)
- Improved debugger (variables, breakpoints)

### Long-term goals

- Sprites and smooth scrolling
- Sound
- Joystick emulation
- Cartridge binaries support

### No-goals

- Making competitive product to Vice and other well-established emulators.
- CRT emulation

## Screenshots

<img src="screenshots/dead_test.png?raw=true" width="300"/>

[The Dead Test cartridge](http://blog.worldofjani.com/?p=164) image executed quite fine, but the two timers
at the bottom-right of the scrren show zeros, which - according to the documentation - means
"Possible 6526 CIA Failure". Well, it's quite right as CIA is not implemented at this stage
at all (apart keyboard support).

<img src="screenshots/diagnostic.png?raw=true" width="300"/>

Another diagnostic tool - this one - besides proving that I've found the right Unicode characters
for C64 graphics keys, seems to be failing miserably. At least it's doing nithing (perhaps depends
heavily on clock).

<img src="screenshots/cli-emu.png?raw=true" width="300"/>

This is how it looks like when run the raw 6502 "machine" emulator in CLI.
It's configured as 1kB machine that starts loaded program at 0x200. The provided ROM
is 6-byte jump vector setting the reset vector to 0x200. Some debugging
is possible that way.

## Credits

- [srounce](https://github.com/srounce) - made the environment work with Nix Flakes. So cool!

## References

I've compiled much longer list of topic-specific links in a separate
[references document](docs/references.md), but here I'd like to mention a few sites
I was returning to constantly in order to gain my knowledge about 6502 and C64 internals.

- [6502.org](http://6502.org/)
  An absolute must-visit page for all 6502 enthusiasts: documents, active forum and plenty
  of links to existing projects, tools, assemblers, etc.
- [C64 Wiki](https://www.c64-wiki.com/wiki/Main_Page)
  Another very detailed knowledge base site, but focused on C64.
  Lot of information about C64-specific chips, Kernal functions, graphics
  and C64 software.
- [Codebase 64](https://codebase64.org/)
  This is a deep-dive into C64 world. Lot of ASM examples, advanced topics
  and many how-to guides.
- [C64OS](https://c64os.com/)
  C64OS is a name of a modern operating system for C64, but the site
  provides much more than that. The author invested a lot of time in understanding
  and explaining the details of how C64/6510 functions and has a great talent
  in presenting that knowledge in digestible form.
- [pagetable.com](https://www.pagetable.com/)
  Great blog and reference docs about 6502, C64 and many other topics.
  It provides super detailed C64 [Memory Map](https://www.pagetable.com/c64ref/c64mem) and
  [ROM Disassembly](https://www.pagetable.com/c64ref/c64disasm/)
