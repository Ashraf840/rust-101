// Tut-5: If-condition

fn main() {
    let age: u8 = 8;
    if (age >= 1) && (age <=18) {
        println!("Important Birthday!");
    } else if (age == 21) || (age ==50) {
        println!("Important Birthday!");
    } else if age >= 65 {
        println!("Important Birthday!");
    } else {
        println!("Not Important Birthday!");
    }
}