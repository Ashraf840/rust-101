// Prac-10: Enum (Implement methods to enums)

/*
 * Objectives
 * - Create an enum.
 * - Implement a method into that enum type data.
 * - Create a variable of that enum type. Define a specifc value from enum while creating that variable.
 * - Check whether the method works or not using that variable.
 */

/*
 * Q/A
 * - What is an enum?
 *      - Like structs, enums are also defined as complex data types. But the difference is, enums grant use to define as set of possibile values.
 *      - To add methods to enums, "impl" is required to be used.
 *      - It's more like choices of certain values stored in a complex data structure.
 */

#![allow(dead_code)]

// Define all weekdays as enum; since there are certain number of days in a weekdays
enum Days {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

// Implement a method which specifies whether it's a weekend or not
impl Days {
    fn is_weekend(&self) -> bool {
        match self {
            // if days are firday/saturday, then it's a weekend
            Days::Friday | Days::Saturday => true,
            _ => false
        }
    }
}

fn main() {
    // Create a enum type var
    let day: Days = Days::Friday;
    println!("Is Weekend: {}", day.is_weekend());
}