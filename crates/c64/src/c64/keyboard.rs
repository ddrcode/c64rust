const KEY_DOWN_CYCLES: u8 = 2;

pub struct Keyboard {
    last_keys: Vec<u8>,
    key_cycles: u8,
    scan_data: Box<[(u8, u8, u8)]>,
}

// https://www.c64-wiki.com/wiki/Keyboard_code
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy)]
pub enum C64KeyCode {
    Delete = 0x00,
    Return = 0x01,
    CursorLR = 0x02,
    F7 = 0x03,
    F1 = 0x04,
    F3 = 0x05,
    F5 = 0x06,
    CursorUD = 0x07,
    Key_3 = 0x08,
    Key_W = 0x09,
    Key_A = 0x0a,
    Key_4 = 0x0b,
    Key_Z = 0x0c,
    Key_S = 0x0d,
    Key_E = 0x0e,
    LShift = 0x0f,
    Key_5 = 0x10,
    Key_R = 0x11,
    Key_D = 0x12,
    Key_6 = 0x13,
    Key_C = 0x14,
    Key_F = 0x15,
    Key_T = 0x16,
    Key_X = 0x17,
    Key_7 = 0x18,
    Key_Y = 0x19,
    Key_G = 0x1a,
    Key_8 = 0x1b,
    Key_B = 0x1c,
    Key_H = 0x1d,
    Key_U = 0x1e,
    Key_V = 0x1f,
    Key_9 = 0x20,
    Key_I = 0x21,
    Key_J = 0x22,
    Key_0 = 0x23,
    Key_M = 0x24,
    Key_K = 0x25,
    Key_O = 0x26,
    Key_N = 0x27,
    Plus = 0x28,
    Key_P = 0x29,
    Key_L = 0x2a,
    Minus = 0x2b,
    Dot = 0x2c,
    Colon = 0x2d,
    At = 0x2e,
    Comma = 0x2f,
    Pound = 0x30,
    Asterix = 0x31,
    Semicolon = 0x32,
    Home = 0x33,
    RShift = 0x34,
    Equal = 0x35,
    UpArrow = 0x36,
    Slash = 0x37,
    Key_1 = 0x38,
    LeftArrow = 0x39,
    Ctrl = 0x3a,
    Key_2 = 0x3b,
    Space = 0x3c,
    Cmd = 0x3d,
    Key_Q = 0x3e,
    Stop = 0x3f,
}

impl Into<u8> for C64KeyCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl C64KeyCode {
    pub fn from_char(ch: char) -> Option<Self> {
        use C64KeyCode::*;
        Some(match ch {
            'a' => Key_A,
            'b' => Key_B,
            'c' => Key_C,
            'd' => Key_D,
            'e' => Key_E,
            'f' => Key_F,
            'g' => Key_G,
            'h' => Key_H,
            'i' => Key_I,
            'j' => Key_J,
            'k' => Key_K,
            'l' => Key_L,
            'm' => Key_M,
            'n' => Key_N,
            'o' => Key_O,
            'p' => Key_P,
            'q' => Key_Q,
            'r' => Key_R,
            's' => Key_S,
            't' => Key_T,
            'u' => Key_U,
            'v' => Key_V,
            'w' => Key_W,
            'x' => Key_X,
            'y' => Key_Y,
            'z' => Key_Z,
            '0' => Key_0,
            '1' => Key_1,
            '2' => Key_2,
            '3' => Key_3,
            '4' => Key_4,
            '5' => Key_5,
            '6' => Key_6,
            '7' => Key_7,
            '8' => Key_8,
            '9' => Key_9,
            '.' => Dot,
            ',' => Comma,
            '/' => Slash,
            ';' => Semicolon,
            ':' => Colon,
            '-' => Minus,
            '=' => Equal,
            ' ' => Space,
            _ => return None,
        })
    }
}

// see https://c64os.com/post/howthekeyboardworks
impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            last_keys: Vec::new(),
            key_cycles: 0,
            scan_data: Box::new([
                (0x3f, 0x7f, 0x7f),
                (0x3e, 0x7f, 0xbf),
                (0x3d, 0x7f, 0xdf),
                (0x3c, 0x7f, 0xef),
                (0x3b, 0x7f, 0xf7),
                (0x3a, 0x7f, 0xfb),
                (0x39, 0x7f, 0xfd),
                (0x38, 0x7f, 0xfe),
                (0x37, 0xbf, 0x7f),
                (0x36, 0xbf, 0xbf),
                (0x35, 0xbf, 0xdf),
                (0x34, 0xbf, 0xef),
                (0x33, 0xbf, 0xf7),
                (0x32, 0xbf, 0xfb),
                (0x31, 0xbf, 0xfd),
                (0x30, 0xbf, 0xfe),
                (0x2f, 0xdf, 0x7f),
                (0x2e, 0xdf, 0xbf),
                (0x2d, 0xdf, 0xdf),
                (0x2c, 0xdf, 0xef),
                (0x2b, 0xdf, 0xf7),
                (0x2a, 0xdf, 0xfb),
                (0x29, 0xdf, 0xfd),
                (0x28, 0xdf, 0xfe),
                (0x27, 0xef, 0x7f),
                (0x26, 0xef, 0xbf),
                (0x25, 0xef, 0xdf),
                (0x24, 0xef, 0xef),
                (0x23, 0xef, 0xf7),
                (0x22, 0xef, 0xfb),
                (0x21, 0xef, 0xfd),
                (0x20, 0xef, 0xfe),
                (0x1f, 0xf7, 0x7f),
                (0x1e, 0xf7, 0xbf),
                (0x1d, 0xf7, 0xdf),
                (0x1c, 0xf7, 0xef),
                (0x1b, 0xf7, 0xf7),
                (0x1a, 0xf7, 0xfb),
                (0x19, 0xf7, 0xfd),
                (0x18, 0xf7, 0xfe),
                (0x17, 0xfb, 0x7f),
                (0x16, 0xfb, 0xbf),
                (0x15, 0xfb, 0xdf),
                (0x14, 0xfb, 0xef),
                (0x13, 0xfb, 0xf7),
                (0x12, 0xfb, 0xfb),
                (0x11, 0xfb, 0xfd),
                (0x10, 0xfb, 0xfe),
                (0x0f, 0xfd, 0x7f),
                (0x0e, 0xfd, 0xbf),
                (0x0d, 0xfd, 0xdf),
                (0x0c, 0xfd, 0xef),
                (0x0b, 0xfd, 0xf7),
                (0x0a, 0xfd, 0xfb),
                (0x09, 0xfd, 0xfd),
                (0x08, 0xfd, 0xfe),
                (0x07, 0xfe, 0x7f),
                (0x06, 0xfe, 0xbf),
                (0x05, 0xfe, 0xdf),
                (0x04, 0xfe, 0xef),
                (0x03, 0xfe, 0xf7),
                (0x02, 0xfe, 0xfb),
                (0x01, 0xfe, 0xfd),
                (0x00, 0xfe, 0xfe),
            ]),
        }
    }

    pub fn cycle(&mut self) {
        if self.last_keys.len() > 0 {
            if self.key_cycles == 1 {
                self.last_keys.drain(..);
            }
            self.key_cycles -= 1;
        }
    }

    pub fn key_down(&mut self, ch: u8) {
        self.last_keys.push(ch);
        self.key_cycles = KEY_DOWN_CYCLES;
    }

    fn is_column_scan(&self, c: u8) -> bool {
        c == 0xfe
            || c == 0xfd
            || c == 0xfb
            || c == 0xf7
            || c == 0xef
            || c == 0xdf
            || c == 0xbf
            || c == 0x7f
    }

    fn get_row_code(&self, ch: u8, col: u8) -> u8 {
        let val = self.scan_data.iter().find(|&&x| x.0 == ch && x.1 == col);
        if let Some(x) = val {
            x.2
        } else {
            0xff
        }
    }

    pub fn scan(&self, dc00: u8, dc01: u8) -> u8 {
        if self.last_keys.is_empty() {
            return 0xff;
        };
        if dc00 == 0 {
            return 0;
        };
        if !self.is_column_scan(dc00) {
            return dc01;
        };
        if self.last_keys.len() < 2 {
            self.get_row_code(self.last_keys[0], dc00)
        } else {
            self.last_keys
                .iter()
                .map(|x| self.get_row_code(*x, dc00))
                .reduce(|x, y| x & y)
                .unwrap()
        }
    }
}
