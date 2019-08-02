use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum PESELErrorKind {
    InvalidDoB,                     // invalid birth date, for example 30st of February or 31st of June...
    DoBOutOfRange,                  // date of birth earlier than 1800 or later than 2299
    SizeError,                      // PESEL has to be 11 characters long
    BadFormat                       // PESEL string contains digits only
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
            PESELErrorKind::DoBOutOfRange => "date is out of range!",                           // ok
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

