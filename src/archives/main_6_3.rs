// Tut-6.3: Array (Python like "for loop")

/*
- For loop
- loop through all array-elem & print out all the numbers
*/

fn main() {
    let arr_1 = [1,2,3,4,5,6,7,8,9,50,101,102,104];
    for val in arr_1.iter() {
        println!("Value : {}", val);
    }
}
