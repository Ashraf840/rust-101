// Tut-5.2: Match (alternative of if-condition)

/*
Conditions:
- If age is between 1-8: Important birthday
- If age is 21 or 50: Important birthday
- If age greater than or equal to 65: Important birthday
- Otherwise, not an important birthday
*/

use std::io;

fn main() {
    let mut age: String = String::new();
    println!("How old are you?");
    io::stdin().read_line(&mut age)
        .expect("Don't have an input!");
    let age: u8 = age.trim_end().parse()
        .expect("Age doesn't assing integer value!");

    match age {
        1..=18 => println!("Important birthday!"),          // if; ranging from 1 to 18
        21 | 50 => println!("Important birthday!"),         // else if; 21 or 50
        65..=u8::MAX => println!("Important birthday!"),    // else if; ranging from 65 to max of unsigned 8 ()
        _ => println!("Not important birthday!"),           // everything else
    };
}