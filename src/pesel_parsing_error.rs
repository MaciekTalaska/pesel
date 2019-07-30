use std::error::Error;

#[derive(Debug, PartialEq)]
/// Custom error - used when constructing PESEL from String
pub struct PESELParsingError {
    message: String
}

impl PESELParsingError {
    pub fn new (msg: &str) -> PESELParsingError {
        PESELParsingError{ message: msg.to_string() }
    }
}

impl std::fmt::Display for PESELParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for PESELParsingError {
    fn description(&self) -> &str {
        &self.message
    }
}

