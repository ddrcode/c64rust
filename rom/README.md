# HelloOS

HelloOS is an alternative to the official C64 ROM. As its name suggests -
it doesn't do much, besides displaying welcome message and a blinking cursor.
I may expand its functionality over time, but it's very unlikely it will
ever turn into fully functional operating system.

I created it because I wanted to include some ROM to this repo,
so you could use the emulator straight away, without any external dependencies.
If you have the official ROM (or its alternatives) you would rather prefer to
use it than this toy.

<img src="../screenshots/hello-os.png?raw=true" width="320"/>

## Why not official ROM?

Some emulators are distributed with the official ROM,
so why not this one? Well, the copyright situation is not very clear,
and - at best - the official C64 Kernal and Basic can be considered as
an abandonware. There are claims that Commodore allowed for using the
ROMs on emulators somewhere in the 90s, but there is no way to find that
original agreement. As such, I decided not to include the ROM into this
repository, and - instead - created my own one.

See [Commodore 64 ROM Copyrights](https://www.lemon64.com/forum/viewtopic.php?t=73857)
discussion at lemon64.com, if you are interested to learn more.

## Alternatives

There are, obviously, other (open source) alternatives to my toy system.
Some of them try to exactly mimic the behaviour of the original system.
It is worth checking the [Open ROMs](https://github.com/MEGA65/open-roms)
repo for examples and further explanation of copyright issues.

### C64OS

Very interesting (although not free) alternative system is
[C64OS](https://c64os.com/) created by Gregory Nacu.
It's a modern, actively developed system for C64 enthusiasts.
Be aware that C64OS won't work at this stage with my emulator, as it
requires graphics mode, while my emulator currently works in
the terminal only.
