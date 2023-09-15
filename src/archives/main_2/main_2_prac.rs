// Prac-2: Build simple age increament calculator

/*
".trim_end()" - Returns a string slice with trailing whitespace removed.
".parse()" - Parses this string slice into another type.
*/

use std::io;

fn main() {
    println!("How old are your?");
    let mut age: String = String::new();
    io::stdin().read_line(&mut age)
        .expect("Don't receive any input!");
    let age: u8 = age.trim_end().parse()
        .expect("Age doesn't assign integer value only!");
    println!("How many years you want to increase?");
    let mut incr_val: String = String::new();
    io::stdin().read_line(&mut incr_val)
        .expect("Don't receive any input?");
    let incr_val: u8 = incr_val.trim_end().parse()
        .expect("Increament value is not integer only!");
    let result: u8 = age + incr_val;
    println!("You're age after {} years will be {}.", incr_val, result);
}