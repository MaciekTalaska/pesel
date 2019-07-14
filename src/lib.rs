use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct PESEL {
    raw:        String,  // raw PESEL as &str
    yob:        u8,   // year of birth (could cover 5 centuries)
    mob:        u8,   // month of birth
    dob:        u8,   // day of birth
    random1:    u8,    // some...
    random2:    u8,    // ...random
    random3:    u8,    // ...data
    // TODO: experiment with union here
    gender:     u8,    // biological gender
    checksum:   u8,    // checksum used for validation
}

impl FromStr for PESEL {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut copy = s.clone().to_string();
        let checksum = copy.pop().unwrap().to_digit(10).unwrap() as u8;
        let gender   = copy.pop().unwrap().to_digit(10).unwrap() as u8;
        let random3  = copy.pop().unwrap().to_digit(10).unwrap() as u8;
        let random2  = copy.pop().unwrap().to_digit(10).unwrap() as u8;
        let random1  = copy.pop().unwrap().to_digit(10).unwrap() as u8;


        let yob = copy[0..2].parse::<u8>().unwrap();
        let mob = copy[2..4].parse::<u8>().unwrap();
        let dob = copy[4..6].parse::<u8>().unwrap();

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
}
