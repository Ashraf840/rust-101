// Prac-16.2.1: Struct (Visibility)

// Ref Video: https://www.youtube.com/watch?v=gi0AQ78diSA

/*
 * Objectives
 * - Create & import a struct from an external file (main_16_2_2_prac.rs).
 *      - Create a vairable using this imported struct.
 * - Create another struct in this file (main_16_2_1_prac.rs).
 *      - Use "Composition" (Inheritance is not allowed) to use the imported struct while defining the new struct in this file (main_16_2_1_prac.rs).
 *      - Mutate after defining a composite variable out of the struct.
 */
 
#![allow(dead_code)]
mod main_16_2_2_prac;
// use main_16_2_2_prac::*;     // Import everything
use main_16_2_2_prac::RandomInfo;

fn main() {
    // Create a variable using the imported struct "RandomInfo"
    let var_one = RandomInfo {
        some_int: -12,
        some_bool: true,
    };
    println!("var_one: {:?}", var_one);

    // Define a struct compositing with imported struct "RandomInfo"
    // The following fields are required to be read/used or needs to define "#![allow(dead_code)]"
    #[derive(Debug)]
    struct StructOne {
        some_int: i8,
        some_bool: bool,
        some_string: String,
        random_info: RandomInfo,
    }

    let mut var_two = StructOne {
        some_int: -91,
        some_bool: false,
        some_string: "This is a string".to_string(),
        random_info: RandomInfo {
            some_int: 30,
            some_bool: true,
        },
    };
    println!("var_two: {:?}", var_two);

    // Modified composite fields of the struct type variables
    var_two.some_bool = true;
    var_two.random_info.some_int = -30;
    println!("var_two: {:?}", var_two);
}