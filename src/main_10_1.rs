// Tut-10.1: Enum (use match on enum)

/*
 * Objectives
 * - Create an enum of the days of week.
 * - Create a variable of that enum type.
 * - Define a match to execute specific operation based on the value assigned to the variable from the enum.
 */

#![allow(dead_code)]

enum Days {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    // Define a var of "Days" enum type
    let day: Days = Days::Sunday;

    match day {
        Days::Sunday => println!("It's Sunday!"),
        Days::Monday => println!("It's Monday!"),
        Days::Tuesday => println!("It's Tuesday!"),
        Days::Wednesday => println!("It's Webnesday!"),
        Days::Thursday => println!("It's Thursday!"),
        Days::Friday => println!("It's Friday!"),
        Days::Saturday => println!("It's Saturday!"),
    }
}