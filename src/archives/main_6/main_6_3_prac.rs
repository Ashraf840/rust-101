// Prac-6.3: Array (Python like "for loop")

/*
 * Q/A
 * - What is the difference between for loop with & without the "iter()" function?
 */

/*
 * Objectives:
 * - Define an array of fixed type & size
 * - Iterate through only the odd numbered elements using the Python like "for loop"
 * - Array slicing using the "arr_1[..] / arr_1[..X] / arr_1[X..]"
 * */

fn main() {
    // array-syntax -->  arr: [dataType, numOfElem] = [..,..,];
    let mut arr_1: [i32; 13] = [1,2,-3,4,5,-6,-7,8,9,-50,101,-102,104];   // array length: 13; array-index-total: 12
    let mut _arr_indx: usize = 0;    // define as "usize" so that it'll work with the "i8" type
    println!("Array: {:?}", arr_1);
    println!("Array length: {}; Array-index-total: {}", arr_1.len(), arr_1.len()-1);

    // Python like "for loop" using "iter()" function; [Currently not using] Since each element will be modified in the for loop
    for i in arr_1 {
        // Prints out only the odd numbers from the array
        if i % 2 != 0 {
            arr_1[_arr_indx] *=  2;
            println!("{}th index array: {}", _arr_indx, i);
        }
        _arr_indx += 1;
    }

    println!("&arr_1[..]: {:?}", &arr_1[..]);   // Like python slice of native list type data
    println!("&arr_1[..4]: {:?}", &arr_1[..4]);   // first 5 elems (doubled from the original)
    println!("&arr_1[5..]: {:?}", &arr_1[5..]);   // from the first 5th elems till end (doubled from the original)
    println!("Array: {:?}", arr_1);
    println!("Array [1]: {:?}", arr_1[1]);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_slice: &[i32] = &my_array[..3];
    println!("my_array: {:?}", my_array);
    println!("my_slice: {:?}", my_slice);
}
