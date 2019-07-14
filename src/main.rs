use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct PESEL {
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

fn main() {
    // that is just a sample PESEL number taken from Wikipedia article
    let pesel = PESEL::from_str("44051401458").unwrap();

    println!("pesel is: {}", pesel.yob);
    println!("pesel is: {}", pesel.mob);
    println!("pesel is: {}", pesel.dob);
    println!("pesel is: {}", pesel.random1);
    println!("pesel is: {}", pesel.random2);
    println!("pesel is: {}", pesel.random3);
    println!("pesel is: {}", pesel.gender);
    println!("pesel is: {}", pesel.checksum);
    println!("pesel is: {}", pesel.raw);
}
