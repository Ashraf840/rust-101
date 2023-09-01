// Prac-7: Tuple

/*
 * Differences between Tuple & Array
 * - Syntax of tuple: let tuple_1: (dtype1, dtype2,...) = (elem1, elem2,...) 
 *   Syntax of array: let array_1: [dtype, numOfElem] = [elem1, elem2,...]
 * - Tuple's type is defined with "()". Array's type is defined with "[]".
 * - Tuple can be of different types of fixed size. Array is of fixed types & size.
 * - To access each elem of tuple, use "tupleName.indxNum". In contrast, to access each elem of an
 * array, use "arrayName[indxNum]".
 * */

fn main () {
    // Define a tuple of 4 elems of types: i8, String, boolean, &str
    let tuple_1: (i8, String, bool, &str)  = (-13, "Asif".to_string(), true, "This is a &str");
    println!("Tuple: {:?}", tuple_1);

    // Access each elem of tuple each time
    println!("{}th elem: {}", 0, tuple_1.0);

    // Assign tuple elems to other variables (same quantity as tuple elems) in a single line. This
    // variables should be inside a parentheses.
    let (v1, v2, v3, v4) = tuple_1;
    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v3: {}", v3);
    println!("v4: {}", v4);
}
