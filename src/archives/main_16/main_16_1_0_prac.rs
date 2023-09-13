// Prac-16.1.0: Struct

// Ref Video: https://www.youtube.com/watch?v=gi0AQ78diSA

/*
 * Objective
 * - Create a struct. 
 * - Define a variable (Type-1) with this struct.
 * - Create a variable (Type-2) with this struct.
 * - Create a mutable variable (using Type-1) with this struct.
 * */

// #![allow(dead_code)]


fn main() {
    // Define a struct
    #[derive(Debug)]
    struct StructOne {
        some_bool: bool,
        some_int: i8,
        some_string: String,
    }

    // Create a variable (Type-1)
    let var_one = StructOne {
        some_bool: true,
        some_int: -19,
        some_string: String::from("This is a string"),
    };
    println!("var_one: {:?}", var_one);
    println!("var_one.some_bool: {}", var_one.some_bool);
    println!("var_one.some_int: {}", var_one.some_int);
    println!("var_one.some_string: {}", var_one.some_string);

    // Create a variable (Type-2)
    let var_two = StructOne {
        some_bool: false,
        some_int: 50,
        ..var_one
    };
    println!("var_two: {:?}", var_two);

    // Create a mutable vairable (Type-1)
    let mut var_three = StructOne {
        some_bool: false,
        some_int: 50,
        // ..var_one   // Cannot use ..var_one, since it's moved to var_two already & "String" doesn't have the "copy" trait
        some_string: "This is string-2".to_string(),
    };
    var_three.some_bool = true;
    var_three.some_int = 80;
    println!("var_three: {:?}", var_three);
}
