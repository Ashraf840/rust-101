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
    &mut heroes.insert("Superman", "Clark Kent");
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
    // VARIATION-1: get the value of key "Batman" / similar to Pythonic way
    println!("Hero of \"Batman\": {}", heroes["Batman"]);
    
    // VARIATION-1: get the value of key "The Flash" / uisng the rust's HashMap's "get()" func
    let hero_name_option_value: Option<&&str> = heroes.get("The Flash");
    let hero_name_string_value: String = hero_name_option_value.unwrap().to_string();
    println!("Hero of \"The Flash\": {}", hero_name_string_value);

    // Check if a specific key exists in a hashmap before doing further computation

}
