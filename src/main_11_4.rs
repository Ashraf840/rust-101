// Tut-11.4: Vector (Access each values using for loop)

/*
 * Objectives
 * - Create a vector with initial values.
 * - Create a separate variable to track the count of vector index.
 * - Define a for loop & within the definition, print out each value of vector by aceessing them using the temporary variable provided by "for loop" structure.
 */

fn main() {
    // Create a vector with initial values
    let vec_1: Vec<i32> = vec![1,2,3,4,5,6];

    let mut loop_indx: u32 = 0;

    for i in &vec_1 {
        println!("vec_1 {}th value: {}", loop_indx, i);
        loop_indx += 1;
    }
}