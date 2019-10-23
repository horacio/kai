use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TodoErr(pub String);

impl fmt::Display for TodoErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Todo Error: {}", self.0)
    }
}

impl Error for TodoErr {}
