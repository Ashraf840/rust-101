// Tut-15: HashMap

/*
- HashMap: Use to store key-value pairs; like native DICTIONARY IN PYTHON.
- Print a hashmap using the "println!(".... {:?}", HashMap_var)" macro.
*/

/* Q/A
- What is the difference between calling "hashmap.insert(...)" & "&mut hashmap.insert(...)"?
- What is the difference between "for (k, v) in heroes {...}" & "for (k, v) in heroes.iter() {...}" while making through a for-loop?

[Minor]
- What is a macro?
*/


#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Create a new empty HashMap
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    // heroes.insert("Supermap", "Clark Kent");    // [Question is "Q/A" section]
    &mut heroes.insert("Supermap", "Clark Kent");
    &mut heroes.insert("Batman", "Bruce Wayne");
    &mut heroes.insert("The Flash", "Barry Allen");

    // printing a hashmap without using a for-loop to iterate through all the key-value pairs of it.
    println!("Heroes: {:?}", heroes);

    // VARIATION-1: Loop through all the key-value pairs & print them out
    // for (k, v) in heroes {
    //     println!("Key: {}; Value: {}", k, v);
    // }

    // VARIATION-2: Loop through all the key-value pairs & print them out
    for (k, v) in heroes.iter() {
        println!("Key: {}; Value: {}", k, v);
    }

    // Get the length of a hashmap
    println!("Length of \"Heroes\": {}", heroes.len());

    // get the value of a specific key from a hashmap


    // Check if a specific key exists in a hashmap before doing further computation

}
