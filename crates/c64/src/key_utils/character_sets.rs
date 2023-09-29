// C64 provides two separate character sets. One contains low and big caps
// another one graphic characters. Each set is actuall 127 characters
// Characters of 128+ are tha sae, but reversed (background colour)
// References
// https://www.c64-wiki.com/wiki/Character_set
// https://www.pagetable.com/c64ref/charset/

// FIXME  character set 2 misses proper characters after cap Z.

lazy_static! {
    // charset 1 containes inverted characters
    // all chars from 128+ are the same as 0-127, but reversed
    static ref CHAR_SET_1: Vec<char> =
        "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         ─♠🭲🭸🭷🭶🭺🭱🭴╮╰╯🭼╲╱🭽🭾•🭻♥🭰╭╳○♣🭵♦┼🮌│π◥ ▌▄▔▁▏▒▕🮏◤🮇├▗└┐┐┌┴┬┤▎▍🮈🮂🮃▃🭿▖▝┘▘▚\
         @ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         ─♠🭲🭸🭷🭶🭺🭱🭴╮╰╯🭼╲╱🭽🭾•🭻♥🭰╭╳○♣🭵♦┼🮌│π◥ ▌▄▔▁▏▒▕🮏◤🮇├▗└┐┐┌┴┬┤▎▍🮈🮂🮃▃🭿▖▝┘▘▚"
            .chars()
            .collect();

    static ref CHAR_SET_2: Vec<char> =
        "@abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         -ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         @ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         -abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?"
            .chars()
            .collect();
}

pub fn screen_code_to_ascii(code: &u8, set: u8) -> char {
    if set == 0x14 {
        CHAR_SET_1[*code as usize]
    } else {
        CHAR_SET_2[*code as usize]
    }
}
