// Tut-16.1: Struct (Includes method)

// Ref Video: https://www.youtube.com/watch?v=gi0AQ78diSA

/*
 * Objectives
 * - Build a simple struct inside the "main()" func.
 * - Define a variable based on the struct.
 * - Print out the entire struct type var & each field of the var.
 *      - Use double quotations inside the double quotations of "println!()".
 */

#![allow(dead_code)]

fn main() {
    #[derive(Debug)]
    struct StructOne {
        some_pos_int: u8,
        some_neg_int: i8,
        some_string: String,
        some_bool: bool,
    }

    let var_struct = StructOne {
        some_pos_int: 23,
        some_neg_int: -43,
        some_string: "This is a string".to_string(),
        some_bool: false,
    };

    println!("var_struct: {:?}", var_struct);

    // Apply method to the struct "StructOne"
    impl StructOne {
        // Field description of the struct
        fn description(&self) {
            println!(r##"First field: "some_pos_int""##);
            println!(r##"Second field: "some_neg_int""##);
            println!(r##"Third field: "some_string""##);
            println!(r##"Fourth field: "some_bool""##);
        }
    }

    var_struct.description();
}
