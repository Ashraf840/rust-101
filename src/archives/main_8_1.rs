// Tut-8.1: String (Part-1)

/*
- Different ways of creating/instantiating strings.
- Push string/char into a string instance.
- Split the words in a string based on whitespaces.
- Run a for loop using range (like python-range) of 1 to 5 (5-excluding)
- Replace the word "A" from the string-literal.
- Create a string with a default value.
- Iterate through each characters from the string-literal (if in "&str", then convert to "String" using "to_string()" method).
- Convert a string-literal into vector. "Vector.sort()" & "Vector.dedup()".
- Convert a string of "&str" to "String" using "to_string()" method (convert to a heap allocated string).
- Convert "String" data type into array of bytes. Then iterate over each byte & printout.
*/


/*  Q/A
- What is the difference between "String" & "&str"?
- What is "&&str" & "&str"? (Found in the error of concatenating string of data type "&str", but Rust suggest instantiate string of data type "String" instead of "&str".)
- What is the difference between &str and &"It is a string"?
*/


#![allow(unused)]

use std::str::Bytes;

fn main() {
    let my_string_1: String = "Test".to_string();
    let my_string_2: String = "Test".to_owned();    // create an owned String from a borrowed string slice (&str); essentially equivalent to calling "String::from()" with the borrowed string slice as an argument.
    let my_string_3: &str = "Test";
    // println!("string.to_string(): {}", my_string_1);
    // println!("string.to_owned(): {}", my_string_2);
    // println!("&str: {}", my_string_3);

    // create a new growab;e string
    let mut st1: String = String::new();
    st1.push('A');      // str.push()methods expects 'chars' only; throws error if passed string-literal w/ double-quotes
    st1.push_str(" word");  // "str.push_str()" method expects string-literals; throws error if passed a char w/ single-quotes
    st1.push_str(" is a");
    st1.push_str(" combination of characters!");

    println!("st1 : {}", st1);
    // println!("{}", st1.split_whitespace()); // throws error; requires a for loop to interate over
    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }


    // [1] iterate through the words by splitting at any white-space in the string-literal
    // for i in st1.split_whitespace() {
    //     println!("{}", i);
    // }
    
    
    // [2] Run a for loop using range (like python-range) from 1 to 5 (5-excluding)
    // for i in 1..5 {
    //     println!("{}", i)
    // }
    // println!("{}", 1..5);


    // [3] Iterate through 1 to 5 (incld. 5)
    // for i in 1..=5 {
    //     println!("{}", i);
    // }


    // [4] replace the word "A" from the string-literal
    // let mut st2 = st1.replace("A", "Another");  // case-sensitive
    // println!("{}", st2);


    // Create a string using "String" & at the same time assign a value to it; Create a string with a default value
    // create a string of random different chars (but actually a string)
    let mut st3: String = String::from("x r t b k k l o p");
    // println!("Chars: {}", st3.chars());  // throws error; works with a for-loop

    // iterate through each characters from the string-literal
    // update the string to check if each char gets iterated after using "s.char()" method along with a for-loop
    // st3 = "Updated the string!".to_string();    // update the value in "st3"; without the conversion to_string(), it throws an error that expects "String", but found "&str".
    // for ch in st3.chars() {
    //     println!("{}", ch);
    // }


    // Convert a string-literal into vector
    // ----------------------------------------------------- DIDN'T UNDERSTAND VECTOR
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort(); // sort the chars in ascending order
    v1.dedup(); // delete all duplicates
    // println!("Vector elems");
    // for i in v1 {
    //     println!("{}", i);
    // }


    // convert to a heap allocated string
    let st4: &str = "Random String";
    let st5: String = st4.to_string();
    // println!("Heap-allocated string (st5): {}", st5);


    // convert into array of bytes
    let byte_arr1 = st5.as_bytes();
    // println!("Bytes of array: {}", byte_arr1);
    // print!("Array of bytes: ");
    // for ba in byte_arr1 {
    //     print!("{}", ba);
    // }
    // println!();
}
