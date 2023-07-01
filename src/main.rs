// Tut-15: HashMap

/*
- HashMap: Use to store key-value pairs; like native DICTIONARY IN PYTHON.
- Print a hashmap using the "println!(".... {:?}", HashMap_var)" macro.
- Loop through the key & values of a hashmap. (2 variations)
    - for-loop straightly to the hashmap.
    - for-loop to the hashmap.iter() function.
- Length of a hashmap.
- Get the value of a specific key from a hashmap. (Pythonic version)
- get the value of a specific key from a hashmap. (uisng the "get()" func)
- Make futher computations based on the exisitence of key of the hashmap using the 2 variantional approaches:
    - HashMap_var.contains_key(&key_name)
    - "match" expression w/ "Some() ..... None"
*/

/* Q/A
- What is the difference between calling "hashmap.insert(...)" & "&mut hashmap.insert(...)"?
- What is the difference between "for (k, v) in heroes {...}" & "for (k, v) in heroes.iter() {...}" while making through a for-loop?
- What is the difference between "string" and &"string"?
- Difference among &str, &&str, &&&str?

[Minor]
- What is a macro?
- What is the .iter()" function?
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
    
    // VARIATION-2: get the value of key "The Flash" / uisng the rust's HashMap's "get()" func
    let hero_name_option_value: Option<&&str> = heroes.get("The Flash");    // VARIANT-1
    // let hero_name_option_value = heroes.get(&"The Flash");    // VARIANT-2
    let hero_name_string_value: String = hero_name_option_value.unwrap().to_string();   // This is highly discouraged; better appraoch: Use "match" expression with "Some().....None".
    println!("Hero of \"The Flash\": {}", hero_name_string_value);

    // Try to get the value of a non-existing key & handle the scenario of finding a value/None before making any further computation.
    let hero_key_name = "Spiderman"; 
    let key_value = heroes.get(&hero_key_name);
    match key_value {
        Some(x) => println!("key_value: {}", x),
        None => println!("No key is found!"),
    }
    
    // Check if a specific key exists in a hashmap before doing further computation
    let hero_key_name = "Superman"; 
    if heroes.contains_key(&hero_key_name) {
        println!("\"{}\" key exists!", &hero_key_name);
        // get the value using the hashmap-key, since the key exists.
        let key_value = heroes.get(&hero_key_name);
        // println!("key_value: {}", &key_value.unwrap().to_string()); // NB: [DISCOURAGED] Since the scope of getting the value from a hashmap is scopified with checking if the key exists in the hashmap, then it's safe to use the ".unwrap().to_string()" function.
        // [from the prev comment]: BUT IT'S HIGHLY ENCOURAGED TO USE THE "match" EXPRESSION WITH "Some() => ...., None => ...,"
        // Such approach covers if the "key" has a value/ not (if key doesn't exist, then the value will be None) in the hashmap to make further computation.
        // NB: By executing the "match" expression in such approach, it's similar to the if-check of ".contains_key(&key_name)" method.
        match key_value {
            Some(x) => println!("key_value: {}", x),
            None => println!("No key is found!"),
        }
    }
}
