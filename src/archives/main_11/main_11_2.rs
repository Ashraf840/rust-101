// Tut-11.2: Vector (Check value exist using "match" control flow construct)

/*
 * Q/A
 * - What are slice indicies?
 *      - A slice is a view into a contiguous sequence of elements in a collection like an array or a vector.
 * - How to access within a array/vector?
 *      - Slice indices refer to the way you specify which elements you want to access or operate on within a slice.
 *      - ***Slice indicies must be of type "usize".
 */

fn main() {
    // Create a vector type variable, explicitly defining the data type
    let vec_1: Vec<i32> = vec![1,2,3,4,5];
    println!("1st value of vector: {:?}", &vec_1[0]);
    // println!("vec_1 value(1): {:?}", vec_1.get(1).unwrap());    // Check if a value exists; if exists then it'll provide the value wrapping "Some(val)" around the function, otherwise it'll provide "None". But the codebase panics if the get function emits "None"

    // Check if any value exist in the specific index of vector (array) using "match" control flow construct, otherwise None code-block will be performed.
    let vec_index: usize = 4;     // Integer variables which will be used as slice indicies for accessing vectors must be of "usize".
    match vec_1.get(vec_index) {
        Some(value) => println!("Value exists at {}th index: {}", vec_index, value),
        None => println!("No value exists at 5th index!")
    }
}