use crate::machine::Addr;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub addr: Addr,
    pub value: u8
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ${:02x}", self.name, self.value)
    }
}
