// Tut-6.2: Array

/*
- While loop
- loop through all array-elem & print out the even-numbers
*/

fn main() {
    let arr_1 = [1,2,3,4,5,6,7,8,9,50,101,102,104];
    let mut arr_indx: usize = 0;
    // println!("Array length: {}", arr_1.len());
    // println!("{}th elem: {}", arr_1.len(), arr_1[arr_1.len()-1]);   // access the last index dynamically
    while arr_indx < arr_1.len() {
        if arr_1[arr_indx] % 2 != 0 {
            arr_indx += 1;
            continue;
        }
        println!("{}th elem: {}", arr_indx, arr_1[arr_indx]);
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