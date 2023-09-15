// Tut-10: Enum

/*
- enum: Create custom data types with a limited number of potential values.
- We can define functions for enumerated types.
- "impl" keyword: Used to add methods to a "struct"/"enum".
*/

/*
1. Why do I match "self" in the custom method of the enum "Day"?
*/


#![allow(unused)]

fn main() {
    enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }

    // Access different values of enums
    // println!("Enum Day: {}", Day);  // throws error
    // println!("Enum Day: {}", Day::Sunday);  // throws error
    // println!("Enum Day: {}", Day:Sunday);  // throws error

    // A custom method of the enum "Day"
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Friday |  Day:: Saturday => true,
                _ => false
            }
        }
    }

    // Create a variable with data type as enum "Day"
    let today: Day = Day::Sunday;
    // Access the custom method of enum
    println!("Is today a weekend: {}", today.is_weekend());

    // automatically printout different comments (respective to each weekday) based on today
    match today {
        Day::Sunday => println!("Everyone hates Sunday!"),
        Day::Monday => println!("Another exhaustive day!"),
        Day::Tuesday => println!("Donut day!"),
        Day::Wednesday => println!("Hump day!"),
        Day::Thursday => println!("Almost weekend!"),
        Day::Friday => println!("Weekend!"),
        Day::Saturday => println!("Weekend!")
        // _ => println!("Everything else!")    // cover everything else in general
    }
}
