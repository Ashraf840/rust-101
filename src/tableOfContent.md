# Rust Tutorial

Video Ref: [Rust Tutorial Full Course](https://www.youtube.com/watch?v=ygL_xcavzQ4) <br/>
Author: [Derek Banas ](https://www.youtube.com/@derekbanas)

The tutorial files reside inside the "**./src**" dir. <br/><br/>

<h3 align="center"><u>Table of Conent</u></h3>  <br/>

<div align="center">

| File Name   |      Topic      |  Remarks |
|----------|:-------------:|------:|
| **main_0.rs** |  println!(); <br/> print!() | Introduction to <br/> Rust |
| ***main_1.rs** |  #![allow(unused)]; <br/> String; &str; <br/> io::stdin().read_line(); <br/> .expect() | Avoid warnings for unused variables; <br/> Import multiple traits from "**std::io**" module; <br/> Exception handling using "**expect**"; Doc about "**std**", "**crate**", "**module**", "**trait**" |
| **main_1_prac.rs** |  *Same as "main_1.rs"* | Took user-input while handling exception, then printout in the terminal |
| **main_2.rs** |  const; let; .trim(); <br/> .parse(); expect() |  Constant immutable variable; Shadowing using same variable name but with different data types uisng "**.parse()**" method along with exception handling using "**.expect()**";  |
| **main_2_prac.rs** |  Combination of "**main_1.rs**" & "**main_2.rs**" | Get user-input in a string variable then convert it into integer variable of same name (**Shadowing**); happen with another variable; then add those two variables' values |
| **main_0.rs** |  left-aligned | $1600 |
| **main_0.rs** |  left-aligned | $1600 |
| **main_0.rs** |  left-aligned | $1600 |
| **main_0.rs** |  left-aligned | $1600 |
| **main_8_1.rs** |  "string".to_string(); "string".to_owned(); <br/> str.push(); str.push_str(); <br/> st1.split_whitespace(); <br/> for i in 1..5 {}; st1.replace(); str.chars(); st3.chars().collect(); vec.sort(); vec.dedup(); str.as_bytes() | Convert "**&str**" to **String** (*both same for "to_string()" & "to_owned()"*); Push a char value at the end to a string; Push a string value at the end of a string; Split a string slice by whitespace & return an iterator, use "**iter.next()**" or "**for-loop**" to get those string slices; Run a for loop using range (*like python-range*) from 1 to 5 (*5-excluding*); Replace a word with another from a string if exists; loop through each char-of-a-string while using "**str.chars()**"; Convert a "**string.chars**" into "**vector**" then **sort/delete-duplicates**; Convert "**string.chars**" into "**bytes**"  |
| **main_0.rs** |  left-aligned | $1600 |
| **main_0.rs** |  left-aligned | $1600 |
| **main_0.rs** |  left-aligned | $1600 |
| **main_15.rs** |  HashMap::new(); hashmap_var.insert(); println!("{:?}"); for-loop on hashmap_var; for-loop on hashmap_var.iter(); hashmap_var.len(); hashmap_var["keyName"]; hashmap.get("keyName"); .unwrap().to_string(); "match" expression with "Some(x) .... None"; hashmap_var.contains_keys() | Create empty Hashmap; Insert key-value to hashmap; Print out hashmap's key-value using the "**{:?}**" format-specifier; For-loop through the key, value pair of hashmap_var/ [other variant] hashmap_var.**iter()**; Length of hashmap; Get value of hashmap uisng Pythonic way, "**hashmap["keyName"]**"; Get value of hashmap uisng the "**.get()**" method; Handle further computation based on value-existence/None; Unlike being more precise of the situation, almost similar safety-check is performed using the "**hashmap_var.contains_key()**" method w/ if-cond. |

</div>