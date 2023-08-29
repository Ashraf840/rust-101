// Prac-5.3: Using match with compare (cmp)


#![allow(unused_imports)]

use std::io;
use std::cmp::Ordering;

fn main() {
    let mut my_age: String = String::new();
    println!("Enter your age");
    io::stdin().read_line(&mut my_age)
        .expect("Don't have any input!");
    let my_age: u8 = my_age.trim_end().parse()
        .expect("Doesn't contain integer value!");
    let voting_age: u8 = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You cannot vote!"),
        Ordering::Greater => println!("You can vote!"),
        Ordering::Equal => println!("You gain the right to vote!"),
    }
}
