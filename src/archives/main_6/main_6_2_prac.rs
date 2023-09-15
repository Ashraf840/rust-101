// Prac-6.2: Array (while loop)

/*
 * Objectives:
 * - Define an array of fixed type & size
 * - Iterate through all the elements using the "while loop"
 * */

fn main() {
    let array_1: [i8; 8] = [1,2,-3,4,-5,6,7,-8];   // array length: 8; array-index-total: 7
    let mut arr_indx: usize = 0;    // define as "usize" so that it'll work with the "i8" type 
    
    // Initialize a while loop
    while arr_indx < array_1.len() {
        println!("{}th index array: {}", arr_indx, array_1[arr_indx]);
        arr_indx += 1;
    }
}
