// Tut-9: Casting (Type Casting)

/*
- Data type casting using the "as" keyword
*/


#![allow(unused)]

fn main() {
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 7;
    let int3_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);
    println!("Result: {}", int3_u32);
}
