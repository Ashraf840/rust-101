// Tut-11.1: Vector

/*
 * Q/A
 * - What is a vector?
 *      - Vectors are like arrays in other languages.
 *      - If mutable, then they're growable.
 *      - Store multiple values of the same types only.
 * - What is "vec![]"?
 *      - It's a Rust macro which used to create a new vector collection with initial values.
 * What is the differences between Array & Vector in rust?
 */

/*
 * Objectives
 * - Create an immutable empty vector. Display into the console.
 * - Create a non-empty vector with initial values. Display into the console.
 * - Display the length of values of the two vectors.
 * - Access each values of a vector & display them into console.
 */

#![allow(unused)]

fn main() {
    // Create an empty immutable variable
    let vec_1: Vec<i32> = Vec::new();
    println!(r##""vec_1": {:?}"##, vec_1);

    // Create non-empty vector
    let vec_2: Vec<i32> = vec![1,2,3,4,5];
    println!(r##""vec_2": {:?}"##, vec_2);

    // Total length of values in those vectors
    println!("vec_1 length: {:?}", vec_1.len());
    println!("vec_2 length: {:?}", vec_2.len());

    // Access each value of the vector manually
    println!("1st value (vec_2): {:?}", &vec_2[0]);
    println!("2nd value (vec_2): {:?}", &vec_2[1]);
    println!("3rd value (vec_2): {:?}", &vec_2[2]);
    println!("4th value (vec_2): {:?}", &vec_2[3]);
    println!("5th value (vec_2): {:?}", &vec_2[4]);

    // Access the last value of vector "vec_2" dynamically
    println!("Last value (vec_2): {:?}", &vec_2[&vec_2.len()-1]);
}