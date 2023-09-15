// Tut-5.3: Match (std::cmp::ordering)

/*
Conditions:
- If age is between 1-8: Important birthday
- If age is 21 or 50: Important birthday
- If age greater than or equal to 65: Important birthday
- Otherwise, not an important birthday
*/

use std::io;
use std::cmp::Ordering;

fn main() {
    let mut age: String = String::new();
    println!("How old are you?");
    io::stdin().read_line(&mut age)
        .expect("Don't have an input!");
    let age: u8 = age.trim_end().parse()
        .expect("Age doesn't assing integer value!");

    let voting_age: u8 = 18;
    
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote!"),
        Ordering::Greater => println!("Can vote!"),
        Ordering::Equal => println!("Achieve the right to vote!"),
    };
}