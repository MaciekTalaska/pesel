use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum PESELErrorKind {
    InvalidDoB,
    DoBOutOfRange,
    SizeError,
    BadFormat
}

#[derive(Debug, PartialEq)]
/// Custom error - used when constructing PESEL from String
pub struct PESELParsingError {
    message: String,
    kind: PESELErrorKind
}

impl PESELParsingError {
    pub fn new (kind: PESELErrorKind) -> PESELParsingError {
        let msg = match kind {

            PESELErrorKind::BadFormat => "PESEL may only contain digits!",
            PESELErrorKind::DoBOutOfRange => "date is out of range!",
            PESELErrorKind::InvalidDoB => "invalid birth date",
            PESELErrorKind::SizeError => "PESEL has to be of 11 chars long",
        };
        PESELParsingError {
            message: msg.to_string(), kind
        }
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

