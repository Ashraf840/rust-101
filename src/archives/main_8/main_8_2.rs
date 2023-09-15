// Tut-8.2: String (Part-2)

/*
- Slice of string using range-int. Different type sof slicing.
- Delete al values from a string variable using "str.clear()" method.
- String concatenation; Referencing & value-availability (only happens with "" data type).
- Print through each character of a string-literal.
- Print through each converted byte-value of a string-literal.
*/


/*  Q/A
* - Why "st8" doesn't have value unlike "st9" after assigned to "st10". ("st9" is used as "&st9")? 
* - Differences between "String::from()", "...".to_string(), "...".to_owned() 
*/


#![allow(unused)]

use std::str::Bytes;

fn main() {
    // convert to a heap allocated string
    let st4: &str = "Random String";
    let st5: String = st4.to_string();

    // slice of string using range-int
    // println!("***String Slicing***");
    let st6: &str = &st5[0..6];     // starts from beginning until 6th-index
    let st6: &str = &st5[..6];      // starts from beginning until 6th-index
    let st6: &str = &st5[1..6];     // starts from 1st-index until 6th-index
    let st6: &str = &st5[1..];      // starts from 1st-index till the end
    let st6: &str = &st5[..];       // entire string
    let st6: &str = &st5;           // entire string
    // println!("Sliced string: {}", st6);
    // println!("String length: {}", st6.len());

    let mut st7: String = st6.to_string();  // "&str" doesn't have the ".clear()" method; thus used the "String" data type
    // println!("st7 (before deleting values): {}", &st6);
    // Delete values from string
    st7.clear();
    // println!("st6: {}", &st6);
    // println!("st7 (after deleting values): {}", &st7);

    // Combine/Concatenate string 

    // NB: Tried concatenating two string instantiated from "&str"; but throws error;
        // Error-msg: note: string concatenation requires an owned `String` on the left
    let st8: String = String::from("Just some");    // Create a string with a default value
    let st9: String = String::from("words");
    let st10 = st8 + " " + &st9;  // reference to "st9" unlike "st8"; thus "st8" doesn't exist/available now
    // println!("Concatenated string: {}", st10);
    // println!("st9: {}", st9);
    // println!("st8: {}", st8);   // "st8" is not available; since it's moved to "st10"

    // // Testing: if value-availability happens with "int" data type like "String" data type
    // let int_1: u8 = 10;
    // let int_2: u8 = int_1;
    // println!("int_1 value (after assigned to int_2): {}", int_1);   // "int_1" still exist after assigned to "int_2"

    // Loop through each character of "st10"
    // for char in st10.chars() {
    //     println!("{}", char);
    // }

    // Loop through each character of "st10"
    // for byte in st10.bytes() {
    //     println!("{}", byte);
    // }
}
