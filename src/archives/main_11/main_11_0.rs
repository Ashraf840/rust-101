// Tut-11.0: Vector

/*
- Vector: They are like ARRAYS & can grow if mutable.
- ** They can only store values of the same type.
*/

/* Q/A
- Why do I have to do "*i *= 2;" instead of "i *= 2"? (while manipulating values of a vector using for-loop)
*/


#![allow(unused)]

use std::io;

fn main() {
    // Create an empty vector of type signed integer
    let vec1: Vec<i32> = Vec::new();    // i32: -n to n; empty vector
    let mut vec2: Vec<i32> = vec![1,2,3,4]; // vector with defined values
    &vec2.push(5);
    // Print out all the elems of vector "vec2"
    println!("Elements of vec2");
    for i in &mut vec2 {
        println!("{}", i);
    }

    // Access the "n"-index elem of "vec2"
    println!("1st elem: {}", &mut vec2[0]);

    // Mnanipulate each elem of "vec2"; multiple by 2
    for i in &mut vec2 {
        *i *= 2;
    }
    
    println!("Manipulated elems of vec2");
    for i in &mut vec2 {
        println!("{}", i);
    }
}
