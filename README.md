[![Travis Build Status](https://travis-ci.org/MaciekTalaska/pesel.svg?branch=master)](https://travis-ci.org/MaciekTalaska/pesel)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/cjp41x6e7p3xamth?svg=true)](https://ci.appveyor.com/project/MaciekTalaska/pesel)

`PESEL` is a simple Rust library to validate PESEL numbers.


What is PESEL number?
=====

PESEL is national identification number used in Poland. Every citizen of Republic of Poland is assigned PESEL when being born (and since 2015 this should also apply to foreigners living in Poland for more than 2 months). 

There are some interesting facts about the PESEL number - [find more on Wikipedia](https://en.wikipedia.org/wiki/PESEL)

Usage & Examples
=====

This library offers two main features:

a) creating PESEL from String (and performing some checks to make sure PESEL is valid)

```rust
use std::str::FromStr;

let pesel_number = "44051401458";
let result = PESEL::from_str(pesel_number);
match result {
    Ok(pesel) => println!("PESEL: {}", pesel),
    _ => println!("invalid PESEL provided"),
}

// alternatively, pass a string literal:

let result = PESEL::from_str("44051401458");
match result {
    OK(pesel) => println!("PESEL: {}", pesel),
    _ => println!("invalid PESEL string"),
}
```

b) generating PESEL number, based on date of birth of a person and their biological gender

```rust

let result = PESEL::new(1980, 05, 26, PeselGender::Male);
match result {
    Ok(pesel) => println!("generated PESEL: {}", pesel),
    _ => println!("unable to create PESEL for specified date"),
}
```

Note: behavior of this method changed. It used to return PESEL, now returns `Result<PESEL, PeselParsingError>`. Errors are usually due to:
- provided date of birth being out of range for PESEL (earlier than 1800, later than 2299)
- provided date of birth is not valid i.e.: 30th of February, 31st of April...



Please note that after PESEL number structure is constructed there is no way to change it - it stays immutable forever. 

TODO
----

 - [x] validate date before PESEL is created to avoid creation of PESEL for invalid date (31st of February, 31st of April, 31st of June...)
 - [ ] validate PESEL numbers in bulk (ideally: reading from file)
