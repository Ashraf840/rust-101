// Prac-16: Create a basic struct & access its properties/fields.

#![allow(unused)]

fn main() {

    // Struct / custom data type for customer consists of name, address & phone number
    #[derive(Debug)]    // TO PRINTOUT THE VARIABLE PF CUSTOME DATA TYPE (STRUCT)
    struct Customer {
        name: String,
        address: String,
        phone: String,
    }

    // Create a variable of type "Customer"
    let mut bob: Customer = Customer {
        name: String::from("Bob Marley"),
        address: String::from("555 Main St"),
        phone: String::from("+880-1917-739-840")
    };

    // Print out the variable "bob"
    println!("Customer (bob): {:?}", bob);
    // Access all props of the variable "bob"
    println!("Name: {}", bob.name);
    println!("Address: {}", bob.address);
    println!("Phone: {}", bob.phone);

    // Modify prop(s) of variable "bob"
    bob.name = String::from("Bob Harley");
    bob.address = String::from("565 Main St");
    bob.phone = String::from("+880-1111-111-111");

    println!("Customer (bob) - updated: {:?}", bob);
}