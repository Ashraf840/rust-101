// Tut-16: Struct

/*
 * Miscellaneous
 * - Allow "dead_code" initially, since we aren't reading the fields of a struct type variable.
 *
 * Objectives
 * - Define a simple strcut. Build an immutable variable based on top of that struct.
 * - Try to modify the fields of that struct-type variable. Panic the codebase.
 * - Later make the variable as mutable.
 * - Intiate a new variable using an existing variable's values.
 *
 * Q/A
 * - What is a struct?
 *    - Used to define complex data types. Comparing other languages, it's more like "objects".
 *    - A struct might contain methods dedicated to that struct only.
 *    - While defining fields inide a struct skeleton, only define the field names including it's
 *    data type.
 *    - Best parctise while defining name of a struct: CamelCasing
 * */

#[allow(dead_code)]

fn main() {
    #[derive(Debug)]
    struct StructOne {
        some_int: i8,
        some_string: String,
        some_bool: bool,
    }

    // Initially define an immutable variable using this struct ("struct_1")
    let mut var_struct =  StructOne  {
        some_int: -20,
        some_string: "This is a string".to_string(),
        some_bool: true,
    };

    // Print a variable of a struct type; for this, use the "#[derive(Debug)]" attribute which
    // automatically implements the "std::fmt::Debug" trait; which allows to print the struct type
    // variable using the "println!()" macro.
    println!("var_struct: {:?}", var_struct);

    // Until the LOC, the fields of the struct type variable weren't read, thus use the attribute
    // "#[allow(dead_code)]". After this LOC, the attribute isn't required.
    println!("var_struct.some_int: {}", var_struct.some_int);
    println!("var_struct.some_string: {}", var_struct.some_string);
    println!("var_struct.some_bool: {}", var_struct.some_bool);

    // Modify a field while the var "var_struct" is initialized as immutable.
    var_struct.some_int = 100;  // panicked the codebase, since the variable is immutable.

    println!("var_struct.some_int: {}", var_struct.some_int);

    // Initiate another variable using the existing variable of the struct type.
    // NB: Much handy, in terms of just changing a specific field(s) while keeping the rest of the
    // other fields unchanged in the struct type variable. While using an existing variable to fill
    // out the fields of the new var of struct type, negate using the comma (,) at the end.
    let var_struct_2 = StructOne {
        some_int: -128,  // "i8" range limit: -128 to 127
        ..var_struct
    };

    println!("var_struct_2: {:?}", var_struct_2);
}
