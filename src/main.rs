use std::str::FromStr;
use pesel::pesel::{PESEL as PESEL, PeselGender};

fn main() {
    let pesel_number ="44051401458";
    let pesel = PESEL::from_str(pesel_number);
    match pesel {
        Ok(t) => println!("{}", t),
        _ => panic!("invalid PESEL provided")
    }

    println!("--- PESEL generation ----");
    // TODO: not sure it is so nice to have unwrap here, maybe for the sake of consistency with the above example using match & Result<Ok, Err> is a better idea?
    let generated_pesel = PESEL::new(1980, 05, 26, PeselGender::Male ).unwrap();
    println!("generted pesel: {}", generated_pesel);
}
