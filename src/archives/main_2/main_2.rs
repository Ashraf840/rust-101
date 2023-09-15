// Tut-2: Constant & Shadowing
/*
Constant: Define var-name in uppercase; more likely default immutable variables, but unlike 'let', data types required to be explicitly defined.
Shadowing: Define variables of same names but different datatypes.
*/

fn main() {
    // Constant
    const ONE_MIL: u32 = 1_000_000;     // 32-bit unsigned integer type: 0 to 4294967295
    const PI: f32 = 3.1411592;          // 32-bit floating point type
    // Shadowing
    let age: &str = "30";
    let mut age: u8 = age.trim().parse()
        .expect("'age' wasn't assigned a number!"); // ".trim()" is used to deduct any leading/trailing whitespaces; ".parse()" is used for converting data types.
    age = age + 1;  // required since lately defined 'age' as mutable-var
    println!("I'm {} & I want ${}.", age, ONE_MIL);
    println!("PI value: {}", PI);
}