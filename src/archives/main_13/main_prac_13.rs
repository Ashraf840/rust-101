#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::io;

/*
 * Objectives
 * - Create generic function based general math operations
 * - Get user input for two numbers & an operator
 * - Use match function to execute the appropriate genereic function
 * */

fn get_sum<T:Add<Output = T>>(x:T, y:T) -> T { x+y }    // Addition
fn get_sub<T:Sub<Output = T>>(x:T, y:T) -> T { x-y }    // Subtraction
fn get_mul<T:Mul<Output = T>>(x:T, y:T) -> T { x*y }    // Multiplication
fn get_div<T:Div<Output = T>>(x:T, y:T) -> T { x/y }    // Division
fn get_rem<T:Rem<Output = T>>(x:T, y:T) -> T { x%y }    // Remainder

fn main() {
    let mut num_1: String = String::new();
    let mut num_2: String = String::new();
    let mut operator: String =  String::new();
    println!("Enter 1st num:");
    io::stdin().read_line(&mut num_1)
        .expect("No input is passed!");
    println!("Enter 2nd num:");
    io::stdin().read_line(&mut num_2)
        .expect("No input is passed!");
    println!(r##"Enter the operator from: "+", "-", "*", "/", "%"##);
    io::stdin().read_line(&mut operator)
        .expect("No input is passed!");

    // Parse the String-type integers into actual integers (using the same variable/Shadowing)
    let num_1: f64 = num_1.trim().parse()
        .expect("Invalid input: num_1");
    let num_2: f64 = num_2.trim().parse()
        .expect("Invalid input: num_2");

    // println!("operator: {:?}", operator.trim());    // Trims out the CRLF (Carriage return and line feed) as the suffix of user-input
    // println!("operator: {:?}", operator.trim().chars());    // "chars()" method returns an iterator of characters; array of all the elems wrapped w/ "Char" type
    // println!("operator: {:?}", operator.trim().chars().next());  // Selects the first elem; wrapped w/ "Some" type
    
    // Parse the "operator" string into character of "Some(char)" type
    let operator = operator.trim().chars().next();

    // Match the operator which the user wants to perform between the two numbers
    match operator {
        Some('+') => println!("Sum: {}", get_sum(num_1, num_2)),
        Some('-') => println!("Sub: {}", get_sub(num_1, num_2)),
        Some('*') => println!("Mul: {}", get_mul(num_1, num_2)),
        Some('/') => println!("Div: {}", get_div(num_1, num_2)),
        Some('%') => println!("Rem: {}", get_rem(num_1, num_2)),
        _ => println!("Invalid operation!"),
    }
}
