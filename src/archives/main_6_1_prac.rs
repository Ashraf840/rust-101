// Prac-6.1: Array (Rust "loop")

/*
 * Objectives:
 * - Define an array of fixed type & size
 * - Iterate through all the elements using the Rust "loop"
 * */

/*
 * Differences between array & tuples
 * - Syntax of array: let array_1: [dtype, numOfElem] = [elem1, elem2,...]
 *   Syntax of tuple: let tuple_1: (dtype1, dtype2,...) = (elem1, elem2,...)
 * - Array's type is defined with "[]". Tuple's type is defined with "()". 
 * - Array are of fixed type & size. Tuple can be of different types of fixed size.
 * - To accees each elem of array, use "arrayName[indxNum]". To access each elem of tuple, use
 * "tupleName.indxNum".
 * */

fn main() {
    let array_1: [i8; 5] = [1,2,-3,4,-5];   // array length: 5; array-index-total: 4
    let mut arr_indx: usize = 0;    // define as "usize" so that it'll work with the "i8" type 
    // Initially generates an infinite loop unless it's broken conditionally
    loop {
        if arr_indx == array_1.len() {
            break;
        }
        println!("{}th array indexed value: {}", arr_indx, array_1[arr_indx]);
        arr_indx += 1;
    }
}
