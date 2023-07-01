// Tut-14: Ownership

/*
- Assignment operator vs clone operator.
- "Clone" operator doesn't work with integers, boolean, characters, floats, tuples.
- It works with String, array, vector etc.
*/
#![allow(unused)]

use std::io;

// Create a void function to printout a string
fn print_str(x: &str) {
    println!("A happy {}!", x);
}

// Create a function which takes String-args as its params & returns a String
fn return_str(x: &mut String) -> &mut String {
// fn return_str(mut x: String) -> String {
    // change any other word to user-input
    let mut usr_inpt: String = String::new();
    io::stdin()
        .read_line(&mut usr_inpt)
        .expect("Didn't receive input!");
    // x = &mut usr_inpt;
    // x
    x.push_str(&usr_inpt);
    x
}

fn main() {
    // let mut st1: String = String::new();
    let mut st1: String = String::from("World ");
    // let st2 = st1;  // copy; thus st1 doesn't exist afterwards; value moved to "st2"
    // println!("{}", st1);    // after the value gets moved, while borrowing it'll throw an error
    
    let st2 = st1.clone();  // make a clone of "st1"; actually makes a separate copy of "st1" & assigns to "st2"
    // println!("{}", st1);

    // print_str(&st1); // invoke func
    println!("User Input: ");
    // println!("Hellow {}", return_str(&st1));

    // Store the returned-string & modify that with another func
    
    // VARIATION-1: passing args to func
    // let mut return_result = return_str(st1);    // by passing arg like this, the value of "st1" is completely moved to the param of the func "return_str()"; cannot borrow it afterwards.
    // println!("st1: {}", &st1);   // cannot borrow it here, since "st1" is completely moved to the param of "return_str()" func.
    
    // VARIATION-2: passing args to func; place a pointer to the original variable-mem-location; thus if the value passed inside the function gets modified, it also modifies the value of the original variable.
    let mut return_result = return_str(&mut st1);
    println!("return_result: {}", &mut return_result);
    // println!("st1 - after modification: {}", st1);

    // let changed_str: &mut String = change_str(&mut return_result);   // THROWS ERROR NOW; SINCE THE FUNC "change_str()" RETURNS AN OWNED STRING
    let changed_str: String = change_str(&mut return_result);
    println!("changed_str: {}", &changed_str);
}


// [NB]: Rust's borrow checker doesn't allow returning references to values that go out of scope. Thus, modify the function "change_str()" to return an owned "String" instead of a mutable reference (&mut String).
fn change_str(str_param: &mut String) -> String {
    // check if the string-literal contains "test" keyword
    let mut modified_str: String = String::new();
    for word in &mut str_param.split_whitespace() {
        if (word == "test") {
            // println!("{}", word);
            &mut str_param.replace("test", "Wide Web!");    // [issue]: Why the reference of "&mut str_param" using the ".replace()" method is not modifying the original-param-value thereafter?
            &mut modified_str.push_str(&mut str_param.replace("test", "Wide Web!"));    // As a workaround: store/push-at-the-end the modified/replaced string-literal into another mutable string-type variable.
        } 
    }
    println!("str_param: {}", &str_param);
    println!("modified_str: {}", &modified_str);
    modified_str    // rust won't let me to send a referenced mutable string out of the scope of this function. Thus instead of "&mut modified_str", return "modified_str".
}
