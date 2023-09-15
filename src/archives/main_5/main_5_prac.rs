// Prac-5: Get age as user input & display whether his/her birthday is now important or not.
/*
Conditions:
- If age is between 1-8: Important birthday
- If age is 21 or 50: Important birthday
- If age greater than or equal to 65: Important birthday
- Otherwise, not an important birthday
*/

use std::io;

fn main() {
    println!("How old will you be in your next birthday?");
    let mut age: String = String::new();
    io::stdin().read_line(&mut age)
        .expect("Don't receive any input!");
    let age: u8 = age.trim_end().parse()
        .expect("Age is not assigned integer value!");
    // println!("Age: {}", age);
    if (age >= 1) && (age <= 18) {
        println!("Important birhtday!");
    } else if (age == 21) || (age == 50) {
        println!("Important birhtday!");
    } else if age >= 65 {
        println!("Important birhtday!");
    } else {
        println!("Not important birhtday!");
    }
}