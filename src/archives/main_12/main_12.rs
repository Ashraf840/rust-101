// Tut-12: Function

/*
- Function: Define a function before or after the "main" function.
*/

/* Q/A
- 
*/

#![allow(unused)]

// simple function; Void function; Non-Value returning function
fn say_hello() {
    println!("Hellow!");
}

// pass args to a function's param; Void function; Non-Value returning function
fn get_sum(x: i32, y:i32) {
    println!("{} + {} = {}", x, y, x+y);
}

// pass args to a function's param; Single value returning function
fn get_sum2(x: i32, y: i32) -> i32 {
    // return x + y;    // VARIATION-1
    x + y   // VARIATION-2; Since the "return" keyword isn't used, don't use the semi-colon (";")
}

// pass args to a function's param; Multiple values returning function
fn get_sum3(x: i32, y:i32) -> (i32, bool) {
    return (x+y, true);
}

fn main() {
    // say_hello();
    // get_sum(5, 9);
    let x: i32 = 5;
    let y: i32 = 10;
    println!("{} + {} = {}", x, y, get_sum2(x, y));

    // Firstly, store the multiple returned values from the function
    let (sum_result, bool_result) = get_sum3(x, y);     // ASSIGN MULTIPLE VALUES IN SINGLE LINE
    // println!("{} + {} = {}", x, y, sum_result);
    // println!("Boolean result: {}", bool_result);

    // create & pass a vector
    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("Sum list: {}", sum_list(&num_list));
}

// pass a vector (array/list) to func's param; returns a Signed integer value
fn sum_list(list: &[i32]) -> i32 {
    let mut result = 0;

    // for-loop: VARIATION-1
    // for i in list {
    //     result += i;
    // }

    // for-loop: VARIATION-2
    for &i in list.iter() {
        result += &i;
    }
    result
}
