use super::{C64KeyCode, C64ScanCode};

impl From<C64ScanCode> for C64KeyCode {
    fn from(sc: C64ScanCode) -> C64KeyCode {
        sc.key.clone() // :-)
    }
}
