// Prac-16.1.1: Struct (Impl/Implementation)

// Ref Video: https://www.youtube.com/watch?v=gi0AQ78diSA

/*
 * Objectives
 * - Create two types of functions using "impl" of the struct.
 *      - [Dot operator] Using "&self" in the param; which in results in invoking that func using single dot.
 *      - [Double colon] Without using anything in the param; which results in invoking the func w/ double colon (::) along w/ "StructName" as prefix
 * - Create a new field to track down the count of each function of the struct "StructOne".
 */

#![allow(dead_code)]

struct StructOne {
    call_count: u8,
    some_bool: bool,
    some_int: i8,
}

impl StructOne {
    // Invoke this func w/ double colon; since "&self" isn't defined in the param
    // Function available to the type itself which can be accessed using the double colon after mentioning the StructName (Complex data structure).
    fn struct_desc_1() {
        // Self::call_count += 1;   // panicked codebase, if used multiple times.

        println!(r##""StructOne" field description"##);
        println!("some_bool: bool");
        println!("some_int: i8");
    }
    
    // With "&self" we can access and modify data internally
    // Invoke this func w/ a single dot
    fn struct_desc_2(&self) {
        // self.call_count += 1;   // panicked codebase

        println!(r##""StructOne" field description"##);
        println!("some_bool: bool");
        println!("some_int: i8");
    }

    // Compare the initial integer value whether its smaller than the value passed in this func's param while invoking
    fn int_compare(&mut self, val: i8) -> bool {
        self.call_count += 1;   // Track the invoking of the func; This LOC is modifying the struct's field, thus required to define "&mut" before self in the param of this func
        
        self.some_int < val    // Rust compiler knows to return this LOC if no semi-colon is provided
    }
}

fn main() {
    let mut var_one = StructOne {
        call_count: 0,
        some_bool: true,
        some_int: 10,
    };
    
    var_one.struct_desc_2();    // Function uses "&self" as the initial param
    
    StructOne::struct_desc_1();     // Nothing is used like "self" is the param of the func

    println!("Intial value is smaller: {}", var_one.int_compare(19));   // Since the "int_compare()" func is modifying the "call_count" field of the struct tyoe variable "var_one", the var requies to be defined as a mutable variable
    println!("Intial value is smaller: {}", var_one.int_compare(-9));
    println!("Intial value is smaller: {}", var_one.int_compare(5));
    println!(r##"Total count of function's of "StructOne": {}"##, var_one.call_count);
}
