// Tut-11.5: Vector (Mutate values of vector using Rust loop)

/*
 * Objectives
 * - Create a mutable vector with initial values (integers).
 * - Create a "loop_indx" variable to iterate within the Rust loop.
 * - Within the Rust loop, make the values to double.
 * - Lately, print out the entire vector.
 */

fn main() {
    // Create a mutable vector with initial values (integer)
    let mut vec_1: Vec<i32> = vec![1,2,3,4,5,6];

    let mut loop_indx: usize = 0;

    loop {
        if loop_indx == vec_1.len() {
            break
        }
        vec_1[loop_indx] *= 2;
        loop_indx += 1;
    }

    println!("vec_1: {:?}", vec_1);     // mutated; if it's the last LOC, then it isn't required to put a semi-colon at the end
}