pub struct UIState {
    pub addr_from: u16,
}

impl Default for UIState {
    fn default() -> Self {
        UIState { addr_from: 0 }
    }
}
