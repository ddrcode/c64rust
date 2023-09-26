use keyboard_types::KeyboardEvent;

pub struct UIState {
    pub addr_from: u16,
    pub asm_lines: Vec<String>,
    pub key: Option<KeyboardEvent>,
}

impl UIState {
    pub fn new() -> Self {
        UIState {
            addr_from: 0,
            asm_lines: Vec::with_capacity(100),
            key: None,
        }
    }
}
