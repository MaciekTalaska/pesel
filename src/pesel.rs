use crate::pesel_parsing_error::PESELParsingError;
use std::str::FromStr;

const PESEL_LENGTH: usize = 11;

#[derive(Debug)]
pub struct PESEL {
    raw:        String,     // raw PESEL as &str
    yob:        u8,         // year of birth
    mob:        u8,         // month of birth, codes century as well (could cover 5 centuries)
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

impl FromStr for PESEL {
    type Err =  PESELParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != PESEL_LENGTH {
            return Err(PESELParsingError::new("PESEL has to be of 11 chars long"));
        }
        if s.chars().any(|f| !f.is_ascii_digit()) {
            return Err(PESELParsingError::new("PESEL may only contain digits!"));
        }
        // TODO: Q: should PESEL become automatically invalidated (and thus impossible to create) if algorithm based validation fails?
        // This Q above should probably answered NO. This is due to the fact, that some people have been assigned PESEL numbers that do not go through an algorithmic validation, but from the perspective of the State - are still valid. I believe there is a database with all the exceptions stored, and making it more restrictive than it is in real life does not make any sense.
        let checksum = s[10..11].parse::<u8>().unwrap();
        let gender  = s[9..10].parse::<u8>().unwrap();

        // extra validity check in regards to date:
        // a) year could be: 0-99 - no need to check, as it is not possible to code anything more than 99 on 2 decimal places
        let yob = s[0..2].parse::<u8>().unwrap();
        let mob = s[2..4].parse::<u8>().unwrap();
        // a) month could be: 0-12, 20-32, 40-52, 60-72, 80-92
        if (mob > 12 && mob < 20) ||
            (mob > 32 && mob < 40 ) ||
            (mob > 52 && mob < 60) ||
            (mob > 72 && mob < 80) ||
            mob > 92 {
            return Err(PESELParsingError::new("Invalid PESEL! Bad century coded"))
        }
        let dob = s[4..6].parse::<u8>().unwrap();
        // b) day could be max 31
        if dob > 31 {
            return Err(PESELParsingError::new("Invalid PESEL! Day exceeds 31"))
        }

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

impl std::fmt::Display for PESEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "PESEL: {}\n\
        date of birth: {}\n\
        gender: {}\n\
        valid: {}", self.raw, self.date_of_birth(), self.gender_name(), self.is_valid())
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

    pub fn date_of_birth(&self) -> String {
        let century:u16 = match self.mob {
            0...12 => 1900,
            20...32 => 2000,
            40...52 => 2100,
            60...72 => 2200,
            80...92 => 1800,
            _ => panic!("invalid PESEL")
        };
        let year :u16 = self.yob as u16 + century;
        let month = self.mob;
        let day = self.dob;

        format!("{}-{:02}-{:02}", year, month, day)
    }

    pub fn gender_name(&self) -> String {
        match self.is_female() {
            true => format!("female"),
            false => format!("male")
        }
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
        assert_eq!("male", pesel.gender_name());
    }

    #[test]
    fn check_if_is_male() {
        let pesel_input = "44051401458".to_string();

        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();
        assert_eq!(true, pesel.is_male());
        assert_eq!("male", pesel.gender_name());
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

    #[test]
    fn pesel_may_only_contain_digits() {
        let pesel_input = "4405140145a".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str());

        assert_eq!(true, pesel.is_err());
        // TODO: check if Error contains expected message
//        assert_eq!((pesel.expect_err("PESEL may only contain digits!"));
    }

    #[test]
    fn pesel_should_have_proper_century_coded() {
        let pesel_input = "44951201458".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str());

        assert_eq!(true, pesel.is_err());
        // TODO: check if Error contains expected message
    }

    #[test]
    fn birth_day_should_not_exceed_31() {
        let pesel_input = "44053201458".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str());

        assert_eq!(true, pesel.is_err());
        // TODO: check if Error contains expected message
    }

    #[test]
    fn birth_date_should_be_returned_as_ddmmyyyy() {
        let pesel_input = "44051401458".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str()).unwrap();

        assert_eq!("1944-05-14", pesel.date_of_birth());
    }

//    #[test]
//    fn additional_test() {
//        let pesel_input = "44051401459".to_string();
//        let pesel = super::PESEL::from_str(pesel_input.as_str());
//        let result = match pesel {
//            Ok(t) => Some(t),
//            Err() => None,
//        };
//    }
}

