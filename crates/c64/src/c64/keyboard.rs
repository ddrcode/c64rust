use crate::key_utils::C64_SCAN_CODES;

pub struct Keyboard {
    last_keys: Vec<u8>,
    cycle: u8,
}

// see https://c64os.com/post/howthekeyboardworks
impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            last_keys: Vec::with_capacity(5),
            cycle: 5,
        }
    }

    pub fn key_down(&mut self, ck: u8) {
        if !self.last_keys.contains(&ck) {
            self.last_keys.push(ck);
        }
    }

    pub fn key_up(&mut self, ck: u8) {
        if let Some(idx) = self.last_keys.iter().position(|x| *x == ck) {
            self.last_keys.remove(idx);
        }
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
        C64_SCAN_CODES
            .iter()
            .find(|sc| sc.key as u8 == ch && sc.col == col)
            .map_or(0xff, |sc| sc.row)
    }

    pub fn scan(&mut self, dc00: u8, dc01: u8) -> u8 {
        if self.last_keys.is_empty() {
            return 0xff;
        }

        if dc00 == 0 {
            self.cycle -= 1;
            return 0;
        };
        if !self.is_column_scan(dc00) {
            return dc01;
        };

        let val = if self.last_keys.len() < 2 {
            self.get_row_code(self.last_keys[0], dc00)
        } else {
            self.last_keys // when more than one key pressed at the same time
                .iter()
                .map(|x| self.get_row_code(*x, dc00))
                .reduce(|x, y| x & y)
                .unwrap()
        };

        if self.cycle == 0 {
            self.last_keys.remove(0);
            self.cycle = 3;
        }

        val
    }
}
