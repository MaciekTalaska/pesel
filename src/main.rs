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
    match generated_pesel {
        Ok(pesel_number) => println!("generated_pesel: {}", pesel_number),
        _ => println!("unable to create PESEL for specified date")
    }
}
