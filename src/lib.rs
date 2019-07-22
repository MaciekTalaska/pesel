use std::str::FromStr;
use std::error::Error;

const PESEL_LENGTH: usize = 11;

#[derive(Debug)]
pub struct PESEL {
    raw:        String,     // raw PESEL as &str
    yob:        u8,         // year of birth (could cover 5 centuries)
    mob:        u8,         // month of birth
    dob:        u8,         // day of birth
    gender:     u8,         // biological gender
    checksum:   u8,         // checksum used for validation
    // all fields below are used for PESEL validation check
    a:          u8,         // yob (1)
    b:          u8,         // yob (2)
    c:          u8,         // mob (1)
    d:          u8,         // mob (2)
    e:          u8,         // dob (1)
    f:          u8,         // dob (2)
    g:          u8,         // random1
    h:          u8,         // random2
    i:          u8,         // random3
    j:          u8,         // gender
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
            return Err(PESELParsingError::new("PESEL has to be of 11 chars long"));
        }
        let checksum = s[10..11].parse::<u8>().unwrap();
        let gender  = s[9..10].parse::<u8>().unwrap();

        let yob = s[0..2].parse::<u8>().unwrap();
        let mob = s[2..4].parse::<u8>().unwrap();
        let dob = s[4..6].parse::<u8>().unwrap();


        let mut all_chars = s.chars();
        let a = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let b = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let c = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let d = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let e = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let f = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let g = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let h = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let i = all_chars.next().unwrap().to_digit(10).unwrap() as u8;
        let j = all_chars.next().unwrap().to_digit(10).unwrap() as u8;

        Ok(PESEL{
            raw: s.clone().to_string(),
            yob,
            mob,
            dob,
            gender,
            checksum,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
        })
    }
}

impl PESEL {
    pub fn is_valid(&self) -> bool {
        let sum =  9 * self.a +
            7 * self.b +
            3 * self.c +
            self.d +
            9 * self.e +
            7 * self.f +
            3 * self.g +
            self.h +
            9 * self.i +
            7 * self.j;
        self.checksum == (sum % 10)
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
        assert_eq!(true, pesel.is_err());
        // TODO: implement std::cmp::PartialEq, for comparing like below
//        assert_eq!(super::PESELParsingError::new("PESEL has to be of 11 chars long"), pesel);
    }

    #[test]
    fn proper_pesel_should_be_validated() {
        let pesel_input = "44051401458".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();

        assert_eq!(true, pesel.is_valid());
    }

    #[test]
    fn invalid_pesel_should_not_be_validated() {
        let pesel_input = "44051401459".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();

        assert_eq!(false, pesel.is_valid());
    }
}
