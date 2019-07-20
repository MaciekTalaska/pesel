use std::str::FromStr;
use std::error::Error;

const PESEL_LENGTH: usize = 11;

#[derive(Debug)]
pub struct PESEL {
    raw:        String,     // raw PESEL as &str
    yob:        u8,         // year of birth (could cover 5 centuries)
    mob:        u8,         // month of birth
    dob:        u8,         // day of birth
    random1:    u8,         // some...
    random2:    u8,         // ...random
    random3:    u8,         // ...data
    gender:     u8,         // biological gender
    checksum:   u8,         // checksum used for validation
}

#[derive(Debug)]
pub struct PESELParsingError {
    message: String
}

impl PESELParsingError {
    fn new (msg: &str) -> PESELParsingError {
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

impl FromStr for PESEL {
    type Err = PESELParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != PESEL_LENGTH {
//            Err(PESELParsingError {message : "PESEL has to be of 11 chars long".to_string()})
            Err(PESELParsingError::new("PESEL has to be of 11 chars long"))
        }
        else
        {
            let checksum = s[10..11].parse::<u8>().unwrap();
            let gender  = s[9..10].parse::<u8>().unwrap();
            let random3 = s[8..9].parse::<u8>().unwrap();
            let random2 = s[7..8].parse::<u8>().unwrap();
            let random1 = s[6..7].parse::<u8>().unwrap();

            let yob = s[0..2].parse::<u8>().unwrap();
            let mob = s[2..4].parse::<u8>().unwrap();
            let dob = s[4..6].parse::<u8>().unwrap();

            Ok(PESEL{
                raw: s.clone().to_string(),
                yob,
                mob,
                dob,
                random1,
                random2,
                random3,
                gender,
                checksum,
            })
        }
    }
}

impl PESEL {
    pub fn is_valid() -> bool {
        return false;
    }

    pub fn is_male(&self) -> bool {
        self.gender % 2 != 0
    }

    pub fn is_female(&self) -> bool {
        self.gender % 2 == 0
    }

    pub fn year_of_birth() -> u16 {
        return 0;
    }

    pub fn full_date_of_birth() -> u32 {
        return 0;
    }
}

#[cfg(test)]
mod pesel_validator_tests {
    use std::str::FromStr;

    #[test]
    fn building_pesel_from_string() {
        let pesel_input = "44051401458".to_string();

        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();
        assert_eq!( pesel.raw, pesel_input);
        assert_eq!( pesel.yob, 44);
        assert_eq!( pesel.mob, 05);
        assert_eq!( pesel.dob, 14);
    }

    #[test]
    fn check_if_is_female() {
        let pesel_input = "44051401458".to_string();

        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();
        assert_eq!(false, pesel.is_female());
    }

    #[test]
    fn check_if_is_male() {
        let pesel_input = "44051401458".to_string();

        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();
        assert_eq!(true, pesel.is_male());
    }

    #[test]
    fn zero_length_string_should_fail() {
        let pesel_input = "".to_string();

        let pesel = super::PESEL::from_str(pesel_input.as_str());
        //assert_eq!(super::PESELParsingError::new("PESEL has to be of 11 chars long"), pesel.is_err());
        assert_eq!(true, pesel.is_err());
    }
}
