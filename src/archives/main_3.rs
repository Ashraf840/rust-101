// Tut-3: Datatypes (Signed & Unsigned Integers)

/*
Rust is statically typed. All the types must be defined.
Unsigned integer [u8, u16, u32, u64, u128, usize]: ranges from "0 to n"
Signed integer [i8, i16, i32, i64, i128, isize]: ranges from "-n to n"

NB: 'usize' & 'isize' depends on the computer architecture; If it's a 64-bit computer, then it's going to be a 64-bit type variable.
*/

fn main() {
    // Get the MIN/MAX of unsigned integer variables
    println!("u8 (MIN): {}", u8::MIN);
    println!("u8 (MAX): {}", u8::MAX);
    println!("u16 (MIN): {}", u16::MIN);
    println!("u16 (MAX): {}", u16::MAX);
    println!("u32 (MIN): {}", u32::MIN);
    println!("u32 (MAX): {}", u32::MAX);
    println!("u64 (MIN): {}", u64::MIN);
    println!("u64 (MAX): {}", u64::MAX);
    println!("u128 (MIN): {}", u128::MIN);
    println!("u128 (MAX): {}", u128::MAX);
    // since it's 64-bit architecture computer; 'usize' chooses the u64 integer type automatically
    println!("usize (MIN): {}", usize::MIN);
    println!("usize (MAX): {}", usize::MAX);

    // Get the MIN/MAX of signed & unsigned integer variables
    println!("i8 (MIN): {}", i8::MIN);
    println!("i8 (MAX): {}", i8::MAX);
    println!("i16 (MIN): {}", i16::MIN);
    println!("i16 (MAX): {}", i16::MAX);
    println!("i32 (MIN): {}", i32::MIN);
    println!("i32 (MAX): {}", i32::MAX);
    println!("i64 (MIN): {}", i64::MIN);
    println!("i64 (MAX): {}", i64::MAX);
    println!("i128 (MIN): {}", i128::MIN);
    println!("i128 (MAX): {}", i128::MAX);
    // since it's 64-bit architecture computer; 'usize' chooses the u64 integer type automatically
    println!("isize (MIN): {}", isize::MIN);
    println!("isize (MAX): {}", isize::MAX);
    // Floating-point
    println!("f32 (MIN): {}", f32::MIN);
    println!("f32 (MAX): {}", f32::MAX);
    println!("f64 (MIN): {}", f64::MIN);
    println!("f64 (MAX): {}", f64::MAX);
}
