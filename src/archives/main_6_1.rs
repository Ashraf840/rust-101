// Tut-6.1: Array (loop)

/*
- Rust loop
- loop through all array-elem but prints out the odd-numbers
*/

fn main() {
    let arr_1 = [1,2,3,4,5,6,7,8,9,50,101];     // array-length: 11; array-total-index:10, since index starts from 0
    let mut arr_indx: usize = 0;
    println!("Array length: {}", arr_1.len());
    println!("{}th elem: {}", arr_1.len(), arr_1[arr_1.len()-1]);   // access the last index dynamically
    loop {
        // in terms of even-num, increase the index-val & skip the even-numbered array-elem
        if arr_1[arr_indx] % 2 == 0 {
            arr_indx += 1;
            continue;   // skips back to the next loop, so that even numbers can be ignored 
        }
        // Prints out only the odd number from the array
        println!("{}th index-val: {}", arr_indx, arr_1[arr_indx]);
        // break the loop, after it hits the 9th index
        if arr_indx == arr_1.len()-1 {
            break;
        }
        arr_indx += 1;
    }
}


/*
idx - val  - print/skip
0   -  1   -   p
1   -  2   -   s
2   -  3   -   p
3   -  4   -   s
4   -  5   -   p
5   -  6   -   s
6   -  7   -   p
7   -  8   -   s
8   -  9   -   p
9   -  50  -   s
10  -  101 -   p
 */
