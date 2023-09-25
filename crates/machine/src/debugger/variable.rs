use crate::machine::{ Addr };

#[derive(Debug, Default, Clone)]
pub struct Variable {
    pub name: String,
    pub size: usize,
}
