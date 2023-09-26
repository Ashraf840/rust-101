// Tut-16: Struct (aka Structures/Custom Data Structure)

/*
- Struct: A struct is a custom data type storing multiple different types of data.
**We can create variables for each property/field of a "Struct".
*/

#![allow(unused)]

fn main() {
    // Custom data type
    #[derive(Debug)]    // ADD THIS LINE: to printout struct
    struct Customer {
        name: String,
        address: String,
        balance: f64
    }

    // Create a customer with "Customer" data type
    let mut bob: Customer = Customer {
        name: String::from("Bob Martin"),
        address: String::from("555 Main St"),
        balance: 243.50,
    };

    // Print out the entire struct variable
    println!("Struct variable \"bob\": {:?}", bob);

    // Access the value of any field using dot (.) from a variable built on a "struct" / Pythonic way
    println!("Customer name: {}", bob.name);

    // Modify the value of a field in a struct variable
    bob.address = String::from("505 Main St");
    println!("Struct variable \"bob\" (modified): {:?}", bob);
}
