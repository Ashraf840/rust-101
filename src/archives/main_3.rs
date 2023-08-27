// Tut-3: Datatypes (Signed & Unsigned Integers)

/*
Rust is statically typed. All the types must be defined.
Unsigned integer [u8, u16, u32, u64, u128, usize]: ranges from "0 to n"
Signed integer [i8, i16, i32, i64, i128, isize]: ranges from "-n to n"

NB: 'usize' & 'isize' depends on the computer architecture; If it's a 64-bit computer, then it's going to be a 64-bit type variable. Vice versa for 32-bit computer.
*/

fn main() {
    println!("u8 (MIN): {}", u8::MIN);          // u8 MIN: 0
    println!("u8 (MAX): {}", u8::MAX);          // u8 MAX: 255
    println!("u16 (MIN): {}", u16::MIN);        // u16 MIN: 0
    println!("u16 (MAX): {}", u16::MAX);        // u16 MAX: 65535
    println!("u32 (MIN): {}", u32::MIN);        // u32 MIN: 0
    println!("u32 (MAX): {}", u32::MAX);        // u32 MAX: 4294967295
    println!("u64 (MIN): {}", u64::MIN);        // u64 MIN: 0
    println!("u64 (MAX): {}", u64::MAX);        // u64 MAX: 18446744073709551615
    println!("u128 (MIN): {}", u128::MIN);      // u128 MIN: 
    println!("u128 (MAX): {}", u128::MAX);      // u128 MAX: 340282366920938463463374607431768211455
    // since it's 64-bit architecture computer; 'usize' chooses the u64 integer type automatically
    println!("usize (MIN): {}", usize::MIN);    // usize (considering 64-bit) MIN: 0
    println!("usize (MAX): {}", usize::MAX);    // usize (considering 64-bit) MAX: 18446744073709551615

    // Get the MIN/MAX of signed & unsigned integer variables
    println!("i8 (MIN): {}", i8::MIN);          // i8 MIN: -128
    println!("i8 (MAX): {}", i8::MAX);          // i8 MAX: 127
    println!("i16 (MIN): {}", i16::MIN);        // i16 MIN: -32768
    println!("i16 (MAX): {}", i16::MAX);        // i16 MAX: 32767
    println!("i32 (MIN): {}", i32::MIN);        // i32 MIN: -2147483648
    println!("i32 (MAX): {}", i32::MAX);        // i32 MAX: 2147483647
    println!("i64 (MIN): {}", i64::MIN);        // i64 MIN: -9223372036854775808
    println!("i64 (MAX): {}", i64::MAX);        // i64 MAX: 9223372036854775807
    println!("i128 (MIN): {}", i128::MIN);      // i128 MIN: -170141183460469231731687303715884105728
    println!("i128 (MAX): {}", i128::MAX);      // i128 MAX: 170141183460469231731687303715884105727
    // since it's 64-bit architecture computer; 'usize' chooses the u64 integer type automatically
    println!("isize (MIN): {}", isize::MIN);    // isize MIN: -9223372036854775808
    println!("isize (MAX): {}", isize::MAX);    // isize MAX: 9223372036854775807
    // Floating-point
    println!("f32 (MIN): {}", f32::MIN);        // f32 MIN: -340282350000000000000000000000000000000
    println!("f32 (MAX): {}", f32::MAX);        // f32 MAX: 340282350000000000000000000000000000000
    println!("f64 (MIN): {}", f64::MIN);        // f64 MIN: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    println!("f64 (MAX): {}", f64::MAX);        // f64 MAX: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
}
