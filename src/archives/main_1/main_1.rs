// Tut-1: Get input from user

// Mutable & Immutable variable
// By default rust uses IMMUTABLE variables

/*
- Rust Standard Library: The Rust Standard Library is the foundation of portable Rust software, a set of minimal and battle-tested shared abstractions for the broader Rust ecosystem.

- Crate: A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run rustc rather than cargo and pass a single source code file, the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

- Module: A module is a collection of items; functions, structs, traits, impl blocks, and even other modules.

- Trait: A trait in Rust is a group of methods that are defined for a particular type. Traits are an abstract definition of shared behavior amongst different types. So, in a way, traits are to Rust what interfaces are to Java or abstract classes are to C++. A trait method is able to access other methods within that trait.

- TODO: What's the differene between "String" & "&str"?
*/

#![allow(unused)]   // avoid warnings for unused variables

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};    // import nested paths; import multiple traits from "std" Rust's standard library's "io" module.
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    let mut name: String = String::new();   // create new string & assign to 'name' MUTABLE variable
    let greeting: &str = "Nice to meet you";    // assign a string value; IMMUTABLE variable
    // Take user-input; ref to mutable-var (name); read-line returns an enum-type Result (Ok Error);
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input!");

    println!("Hello, {}! {}", name.trim_end(), greeting);   // string-interpolation
}
