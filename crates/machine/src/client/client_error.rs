use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ClientError(String);

impl ClientError {
    pub fn new(msg: &str) -> Self {
        ClientError(String::from(msg))
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine Client error: {}", self.0)
    }
}

impl Error for ClientError {}
