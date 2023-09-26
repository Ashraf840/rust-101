// Tut-13: Generics

/*
- Generics: Mainly used to create functions that can work with multiple different types of data.
- Also it can be used with structs, enums, traits etc.
*/
#![allow(unused)]
// Import "Add" trait, which specifies addition operator for different data types
use std::ops::Add;
use std::io;

// initially write a func that take two values which we don't understand their data types
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    // NB: we cannot use "addition" operator with this form of generics; thus redefine this generics using the "Add" trait
    x + y
}

fn main() {
    println!("5 + 8 = {}", get_sum_gen(5, 8));
    println!("4.3 + 7.6 = {}", get_sum_gen(4.3, 7.6));
}