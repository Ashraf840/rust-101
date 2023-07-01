// Prac-1: Get input from user & prints out a dynamic greeting

#![allow(unused)]

use std::io;

fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you.";
    io::stdin().read_line(&mut name)
        .expect("Don't receive any input!");
    println!("Hellow {}! {}", name.trim_end(), greeting);
}