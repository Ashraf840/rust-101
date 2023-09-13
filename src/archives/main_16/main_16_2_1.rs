// Tut-16.2.1: Struct (Visibility)

// Ref Video: https://www.youtube.com/watch?v=gi0AQ78diSA

/*
 * Objectives
 * - Create a struct in another file (main_16_2_1.rs) & import it in this file (main_16_2_2.rs).
 * - Use composition to propagate fields from foreign struct (resides in another file) into this file's native struct. Since 'inheritance' doesn't exist in Rust.
 * - Create a struct which will include fields from another struct using "Composition".
 * - Access nested fields of the struct type var after it's defined using "Composition".
 * */

#![allow(dead_code)]

mod main_16_2_2;    // Require to access the external Rust file
// use main_16_2_2::RandomInfo;    // Only import the "RandomStruct" struct
use main_16_2_2::*;

fn main() {
    // Create a variable of type struct "RandomInfo"
    let var_struct = RandomInfo {
        some_int: 19,
        some_bool: false,
    };
    println!("var_struct: {:?}", var_struct);

    // Create a struct which will use 'Composition' to include the fields of "RandomInfo" struct
    // NB: Alternative of inheritance over composition.
    
    // #![allow(dead_code)]     // Since the no field is read from the variable build on this struct
    #[derive(Debug)]
    struct StructOne {
        some_int_parent: u8,
        some_string_parent: String,
        some_bool_parent: bool,
        random_info: RandomInfo,
    }
    
    // Create an immutable variable of this composite type struct
    // Require to explicitly define the "#[derive(Debug)]" attribute to print out the struct type
    // variable in the console.
    let var_struct_2 = StructOne {
        some_int_parent: 91,
        some_string_parent: "This is another string!".to_string(),
        some_bool_parent: false,
        random_info: RandomInfo {
            some_int: -72,
            some_bool: true,
        },
    };

    println!("var_struct_2: {:?}", var_struct_2);

    // Access each field of the "var_struct_2" struct. To include the composite fields make
    // reference of foreign struct.
    println!("var_struct_2.some_int_parent: {:?}", var_struct_2.some_int_parent);
    println!("var_struct_2.some_string_parent: {:?}", var_struct_2.some_string_parent);
    println!("var_struct_2.some_bool_parent: {:?}", var_struct_2.some_bool_parent);
    println!("var_struct_2.some_int: {:?}", var_struct_2.random_info.some_int);
    println!("var_struct_2.some_bool: {:?}", var_struct_2.random_info.some_bool);
}
