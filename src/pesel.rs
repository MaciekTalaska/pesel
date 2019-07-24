use crate::pesel_parsing_error::PESELParsingError;
use std::str::FromStr;

use rand::Rng;
use rand::prelude::ThreadRng;

const PESEL_LENGTH: usize = 11;

pub enum PeselGender {
    Male,
    Female,
}

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


impl PESEL {
    pub fn new(year: u16, month: u8, day: u8, pesel_gender: PeselGender) -> PESEL {
        // TODO: what to do if dob is out of accepted range?
//        if year < 1800 && year > 2299 {
//            Err(PESELParsingError::new("date is out of range!"))
//        }
        let pesel_year = year % 100;
        let pesel_month = month + PESEL::calc_month_century_offset(year);

        let mut rng = rand::thread_rng();
        let random1 = rng.gen_range(0,10) as u8;
        let random2 = rng.gen_range(0,10) as u8;
        let random3 = rng.gen_range(0,10) as u8;

        let gender = PESEL::generate_gender_digit(pesel_gender, &mut rng);

        let pesel_string =  format!("{:02}{:02}{:02}{:1}{:1}{:1}{:1}", pesel_year, pesel_month, day, random1, random2, random3, gender);

        let (a, b, c, d, e, f, g, h, i, j) = PESEL::extract_pesel_factors(pesel_string);
        let checksum = PESEL::calc_checksum(a, b, c, d, e, f, g, h, i, j);

        let pesel_string_complete =  format!("{:02}{:02}{:02}{:1}{:1}{:1}{:1}{:1}", pesel_year, pesel_month, day, random1, random2, random3, gender, checksum );

        PESEL::from_str(pesel_string_complete.as_str()).unwrap()
    }
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
        // Q: should PESEL become automatically invalidated (and thus impossible to create) if algorithm based validation fails?
        // The answer for the above question should be NO. This is due to the fact, that some people have been assigned PESEL numbers that do not go through an algorithmic validation, but from the perspective of the State - are still valid. I believe there is a database with all the exceptions stored, and making it more restrictive than it is in real life does not make any sense.
        let checksum = s[10..11].parse::<u8>().unwrap();
        let gender  = s[9..10].parse::<u8>().unwrap();

        // Extra validity check in regards to date:
        // a) year could be: 0-99 - no need to check, as it is not possible to code anything more than 99 on 2 decimal places
        let yob = s[0..2].parse::<u8>().unwrap();
        let mob = s[2..4].parse::<u8>().unwrap();
        // a) month could be: 0-12, 20-32, 40-52, 60-72, 80-92
        if (mob > 12 && mob < 20) ||
            (mob > 32 && mob < 40 ) ||
            (mob > 52 && mob < 60) ||
            (mob > 72 && mob < 80) ||
            mob > 92 {
            return Err(PESELParsingError::new("Invalid PESEL! Only dates between 1800 and 2299 are valid!"))
        }
        let dob = s[4..6].parse::<u8>().unwrap();
        // b) day could be max 31
        if dob > 31 {
            return Err(PESELParsingError::new("Invalid PESEL! Day exceeds 31"))
        }

        let (a, b, c, d, e, f, g, h, i, j) = PESEL::extract_pesel_factors(s.to_string());

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
    fn calc_month_century_offset(year: u16) -> u8 {
        let century = match year {
            1800...1899 => 80,
            1900...1999 => 0,
            2000...2099 => 20,
            2100...2199 => 40,
            2200...2299 => 60,
            _ => 0,
        };
        century
    }

    fn generate_gender_digit(pesel_gender: PeselGender, rng: &mut ThreadRng) -> u8 {
        let women = vec![0, 2, 4, 6, 8];
        let men = vec![1, 3, 5, 7, 9];
        let gender = match pesel_gender {
            PeselGender::Male => men[rng.gen_range(0, 5)] as u8,
            PeselGender::Female => women[rng.gen_range(0, 5)] as u8,
        };
        gender
    }

    fn calc_checksum(a: u8, b: u8, c:u8, d:u8, e:u8, f:u8, g:u8, h:u8, i:u8, j:u8) -> u8 {
        let sum:u16 = 9 * a as u16 +
            7 * b as u16 +
            3 * c as u16 +
            d as u16 +
            9 * e as u16 +
            7 * f as u16 +
            3 * g as u16 +
            h as u16 +
            9 * i as u16 +
            7 * j as u16;
        (sum % 10) as u8
    }

    fn extract_pesel_factors(pesel_string: String) -> (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) {
        let mut all_chars = pesel_string.chars();
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

        (a, b, c, d, e, f, g, h, i, j)
    }

    pub fn is_valid(&self) -> bool {
        let calculated_checksum = PESEL::calc_checksum(self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i, self.j);

        self.checksum == calculated_checksum
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
    use crate::pesel::PeselGender;

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

    #[test]
    fn additional_test() {
        let pesel_input = "44051401459".to_string();
        let pesel = super::PESEL::from_str(pesel_input.as_str());
        let result = match pesel {
            Ok(t) => Some(t),
            Err(_e) => None,
        };

        assert_eq!(false, result.unwrap().is_valid());
    }

    #[test]
    fn generated_pesel_should_be_valid() {
        let should_be_female = true;
        let pesel = super::PESEL::new(1981, 06, 27, PeselGender::Female);

        println!("pesel.checksum: {}", pesel.checksum);
        assert_eq!(true, pesel.is_valid());
        assert_eq!(should_be_female, pesel.is_female());
//        assert_eq!(false, pesel.is_male());
//        assert_eq!(true, pesel.is_female());
    }

    #[test]
    fn generated_pesel_should_have_proper_gender_set() {
        let should_be_female = true;
        let pesel = super::PESEL::new(1981, 06, 27, PeselGender::Female);

        assert_eq!(should_be_female, pesel.is_female());
        assert_eq!("female", pesel.gender_name());
    }

    #[test]
    fn generated_pesel_should_have_proper_gender_set2() {
        let should_be_female = false;
        let pesel = super::PESEL::new(1981, 06, 27, PeselGender::Male);

        assert_eq!(should_be_female, pesel.is_female());
        assert_eq!("male", pesel.gender_name());
    }

    #[test]
    fn generated_pesel_should_print_proper_birth_date() {
        let pesel = super::PESEL::new(1981, 06, 27, PeselGender::Female);

        assert_eq!("1981-06-27", pesel.date_of_birth());
    }

    #[test]
    fn check_for_add_with_overflow() {
        // This test is very specific. It makes sure, that generated pesel, containing many high values (digits) will not result in overflow when calculating checksum
        let year = 2299;
        let month = 12;
        let day = 31;

        let pesel = super::PESEL::new(year, month, day, PeselGender::Male);

        assert_eq!(true, pesel.is_valid());
    }
}

