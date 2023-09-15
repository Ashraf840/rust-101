// Tut-4: Math

/*
f32: 32-bit floating-point type provides 7-digits of precision after decimal point.
f64: 64-bit floating-point type provides 16-digits of precision after decimal point.
*/

fn main() {
    // Precision measurement
    let num_1: f32 = 1.111111111111111;
    println!("f32 precision: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 precision: {}", num_2 + 0.111111111111111);

    // Basic math operator
    let mut num_3: u8 = 5;
    let num_4: u8 = 4;
    println!("5 + 4: {}", num_3 + num_4);
    println!("5 - 4: {}", num_3 - num_4);
    println!("5 * 4: {}", num_3 * num_4);
    println!("5 / 4: {}", num_3 / num_4);
    println!("5 % 4: {}", num_3 % num_4);

    // If "num_3" is mutable-type, then we can do the following.
    num_3 += 50;
    println!("New 'num_3': {}", num_3);
}