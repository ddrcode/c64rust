pub struct UIState {
    pub addr_from: u16,
    pub asm_lines: Vec<String>,
}

impl UIState {
    pub fn new() -> Self {
        let asm_lines = Vec::with_capacity(100);
        UIState {
            addr_from: 0,
            asm_lines,
        }
    }
}
