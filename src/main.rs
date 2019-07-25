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
    let generated_pesel = PESEL::new(1980, 05, 26, PeselGender::Male );
    println!("generted pesel: {}", generated_pesel);
}
