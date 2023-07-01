// Tut-3.1: Datatypes (Floating-point, Boolean, Character)

/*
Floating-Point
Boolean
Character
Unsigned integer (Inside "main_3.rs" file)
Signed integer (Inside "main_3.rs" file)
*/

fn main() {
    // Unused variables; make rust-compiler ignore the warning ragrding unused variable(s)
    let _is_true = true;     // Put an underscore ("_") as prefix of the var-name; thus the rust-compiler can ignore the unused variable
    let _its_string: &str = "Strings are enclosed with double quotes";
    let _its_char: char = 'A';      // A character enclosed with single quotes

    // Used vairables
    let is_false: bool = false;
    let temperature: f32 = 36.5;
    let grade: char = 'A';
    let sent: &str = "This is a sentence!";
    println!("Boolean value: {}", is_false);
    println!("Floating-point value: {}", temperature);
    println!("Character value: {}", grade);
    println!("String value: {}", sent);
}
