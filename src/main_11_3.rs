// Tut-11.3: Vector (Rust Loop to access each value within Vector)

/*
 * Objectives
 * - Create a vector with initial values.
 * - Create a "usize" integer type variable for Rust loop.
 * - Define a Rust loop & within the definition, print out each value of vector by aceessing them using the "loop_indx".
 */

fn main() {
    // Create a vector with initial values
    let vec_1: Vec<i32> = vec![1,2,3,4,5,6];
    println!("vec_1 length: {}", vec_1.len());

    let mut loop_indx: usize = 0;

    // Print out all the values of "vec_1" using a Rust loop
    loop {
        if loop_indx == vec_1.len() {
            break;
        }
        println!("vec_1 {:?}th value: {:?}", loop_indx, &vec_1[loop_indx]);
        loop_indx += 1;
    }
}