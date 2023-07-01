// Tut-5.1: Ternary Operator (if-condition)

use std::io;

fn main() {
    let mut age: String = String::new();
    println!("How old are you?");
    io::stdin().read_line(&mut age)
        .expect("Don't have an input!");
    let age: u8 = age.trim_end().parse()
        .expect("Age doesn't contain integer value only!");
    let can_vote: bool = if age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);
}