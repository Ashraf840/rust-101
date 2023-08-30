// Prac-6: Array

/*
 * Objectives:
 * - Define array with fixed data type & size
 * - Print out the all the elements of array
 * - Access single element at each time using array index statically & dynamically.
 * */

fn main() {
    let array_1: [i32; 6] = [1,2,3,4,5,6];
    println!("array: {:?}", array_1);   // prints out the entire array; doesn't support the defualt formatter "{}"
    println!("Array length: {}", array_1.len());
    // Access single element each time from the array
    println!("{}th index of array (statically access 0th index): {}", 0, array_1[0]);
    println!("{}th index of array (dynamically access 0th index): {}", (array_1.len()-array_1.len()), array_1[array_1.len()-array_1.len()]);
    println!("{}th index of the array (static): {}", 6, array_1[5]);
    println!("{}th index of the array (dynamic): {}", (array_1.len()), array_1[array_1.len()-1]);
}

