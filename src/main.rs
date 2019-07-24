use std::str::FromStr;
use pesel::pesel::PESEL as PESEL;

fn main() {
    let pesel_number ="44051401458".to_string();
    let pesel = PESEL::from_str(pesel_number.as_str());
    match pesel {
        Ok(t) => println!("{}", t),
        _ => panic!("invalid PESEL provided")
    }

    println!("--- PESEL generation ----");
    let generated_pesel = PESEL::new(1980, 05, 26, true);
    println!("generted pesel: {}", generated_pesel);
}
