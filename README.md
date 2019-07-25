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

fn some_function() {
    let pesel_number ="44051401458";
    // you could also use literal as argument for from_str:
    // let pesel = PESEL::from_str("44051401458");
    let pesel = PESEL::from_str(pesel_number);
    match pesel {
        Ok(t) => println!("{}", t),
        _ => panic!("invalid PESEL provided")
    }
}
```

b) generating PESEL number, based on date of birth of a person and their biological gender

```rust

fn some_other_function() {
    let generated_pesel = PESEL::new(1980, 05, 26, PeselGender::Male);
    println!("generted pesel: {}", generated_pesel);
}
```


TODO
----

 - [ ] validate PESEL numbers in bulk (ideally: reading from file)
