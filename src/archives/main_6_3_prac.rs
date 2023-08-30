// Prac-6.3: Array (Python like "for loop")

/*
 * Objectives:
 * - Define an array of fixed type & size
 * - Iterate through only the odd numbered elements using the Python like "for loop"
 * */

fn main() {
    let arr_1: [i8; 13] = [1,2,-3,4,5,-6,-7,8,9,-50,101,-102,104];   // array length: 13; array-index-total: 12
    let mut _arr_indx: usize = 0;    // define as "usize" so that it'll work with the "i8" type
    println!("Array: {:?}", arr_1);
    println!("Array length: {}; Array-index-total: {}", arr_1.len(), arr_1.len()-1);

    // Python like "for loop" using "iter()" function
    for i in arr_1.iter() {
        // Prints out only the odd numbers from the array
        if i % 2 != 0 {
            println!("{}th index array: {}", _arr_indx, i);
        }
        _arr_indx += 1;
    }
}
