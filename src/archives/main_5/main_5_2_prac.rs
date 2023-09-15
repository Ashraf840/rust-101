// Prac-5.2: Match (alternative of if-condition)
// Using 'match' to build a basic calculator

#![allow(unused)]

use std::io;

fn main() {
    let mut first_num: String = String::new();
    let mut second_num: String = String::new();
    let mut operator: String = String::new();
    println!("Enter first num:");
    io::stdin().read_line(&mut first_num)
        .expect("Don't have any input!");
    let first_num: f32 = first_num.trim_end().parse()
        .expect("Doesn't contain only integer value!");
    println!("Enter second num:");
    io::stdin().read_line(&mut second_num)
        .expect("Doesn't contain only integer value!");
    let second_num: f32 = second_num.trim_end().parse()
        .expect("Don't have any input!");
    println!("Enter an operator (+, -, *, /):");
    io::stdin().read_line(&mut operator)
        .expect("Don't have any input!");
    let operator = operator.trim().chars().next();

    // println!("First num: {:?}", first_num);
    // println!("Second num: {:?}", second_num);
    // println!("Operator: {:?}", operator);

    match operator {
        Some('+') => println!("Addition result: {}", (first_num+second_num)),
        Some('-') => println!("Subtraction result: {}", (first_num-second_num)),
        Some('*') => println!("Multiplication result: {}", (first_num*second_num)),
        Some('/') => println!("Division result: {}", (first_num/second_num)),
        _ => println!("Invalid operator!"),
    }
}