// Tut-7: Tuple

/*
Tuple: can contain mutliple data types at once, but require to statically pre-define the types which are going to be used in the tuple.
*/

fn main() {
    let tuple_1: (u8, String, f32) = (30, "Darek".to_string(), 60.3);
    // println!("tuple val: {}", tuple_1);  // cannot print out the whole tuple using "println!()" function.
    
    // Access tuple elems
    // println!("1st element: {}", tuple_1.0);
    // println!("2nd element: {}", tuple_1.1);
    // println!("3rd element: {}", tuple_1.2);

    // Assign values to multiple different tuples at the same time;
    // my understanding: assign tuple-values to multiple different vars at the same time; but the vars need to be inside of a tuple while getting assigned values from the parent tuple.
    let (v1, v2, v3) = tuple_1;
    println!("Age: {}", v1);
    println!("Name: {}", v2);
    println!("Calc result: {}", v3);
}

