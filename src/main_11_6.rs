// Tut-11.6: Vector (Push, Insert, Pop)

fn main() {
    // Create a mutable & growable vector
    let mut vec_1: Vec<i32> = vec![1,2,3,4,5,6];
    
    // Insert a value at the end of the vector
    vec_1.push(7);
    // println!("vec_1: {:?}", vec_1);     // Output:  [1, 2, 3, 4, 5, 6, 7]

    // Insert a value at the beginning of the vector using "insert(index, value)"
    vec_1.insert(0, 18);
    // println!("vec_1: {:?}", vec_1);     // Output:  [18, 1, 2, 3, 4, 5, 6, 7]

    // Pop out the last value from the vector
    let poped_out_value: Option<i32> = vec_1.pop();     // NB: When a value is popped out from a vector, it's wrapped with an "Option" type variant called "Some()", thus the variable should be defined as type "Option<nativeType>".
    println!("poped_out_value (Wrapped): {:?}", poped_out_value);     // Value is wrapped around with "Some()" type "Option" variant.
    println!("poped_out_value (Unwrapped): {:?}", poped_out_value.unwrap());
}