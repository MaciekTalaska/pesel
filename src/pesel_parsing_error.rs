use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum PeselError {
    InvalidDoB,
    DoBOutOfRange,
    SizeError,
    BadFormat,
}

impl PeselError {
    pub fn new (kind: PeselError) -> PeselError {
        kind
    }

    pub fn pesel_error_to_message(&self) -> &str {
        match *self {
            PeselError::InvalidDoB => "Invalid birth date!",
            PeselError::DoBOutOfRange => "Date is out of range!",
            PeselError::SizeError => "PESEL has to be of 11 chars long!",
            PeselError::BadFormat => "PESEL may only contain digits!",
        }
    }
}

impl std::fmt::Display for PeselError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.pesel_error_to_message())
    }
}

impl Error for PeselError {
    fn description(&self) -> &str {
        &self.pesel_error_to_message()
    }
}

