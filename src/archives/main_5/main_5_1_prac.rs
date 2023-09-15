// Prac-5.1: Get user input & use ternary operator based on that input

#![allow(unused)]

use std::io;

fn main() {
    println!("Enter your age");
    let mut age: String = String::new();
    // Reads a line of input & append it to the specified buffer
    io::stdin().read_line(&mut age)
        .expect("Don't have any input!");
    // Convert the input-string-value to integer
    let age: u8 = age.trim_end().parse()
        .expect("Doesn't contain only integer type string!");
    let can_vote: bool = if age >= 18 {
        true
    } else {
        false
    };
    
    if can_vote {
        println!("You can vote");
    } else {
        println!("You cannot vote");
    }
}

