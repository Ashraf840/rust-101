// Tut-4.1: Random Number

/*
Defining the range of random-value-generator
    The ceil-value should be n+1, since the rust-random-value-generator goes upto the definined ceil-num without including the ceil-num
*/

use rand::{Rng};

fn main() {
    // Generate random value ranging from 1 to 100
    let random_num: u8 = rand::thread_rng().gen_range(1..101);
    println!("Random value (1-100): {}", random_num);
}