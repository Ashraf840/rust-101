// Tut-16.1.2.0: Struct (Augment/extend methods/impl of Struct)

/*
 * Objectives
 * - Define a struct into an external file (main_16_1_2_1.rs). 
 * - Define it's initial method(s) into that external file.
 * - Import the struct into this file (main_16_1_2_0.rs). 
 * - Augmented the struct's implementation in this file.
 */

// import the struct "StructOne" using the "mod" func
mod main_16_1_2_1;
use main_16_1_2_1::StructOne;
// use main_16_1_2_1::*;    // Import everything

// Augemnt the impl of the struct "StructOne"
impl StructOne {
    fn is_bigger(&self, val: i8) -> bool {
        self.some_int > val
    }
}

fn main() {
    let var_one = StructOne {
        some_bool: true,
        some_int: -41,
    };
    println!("Initial val is smaller than {}: {:?}", 9, var_one.is_smaller(9));
    println!("Initial val is bigger than {}: {:?}", 128, var_one.is_bigger(-128));
}