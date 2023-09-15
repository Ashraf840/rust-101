// Tut-7.1: For looping in tuple

fn main () {
    // Define a tuple of 4 elems of types: i8, String, boolean, &str
    let tuple_1: (i8, String, bool, &str)  = (-13, "Asif".to_string(), true, "This is a &str");
    println!("Tuple 1: {:?}", tuple_1);

    let _loop_count: u8 = 0;
    
    // For loop to iterate all the elems of tuple 1
    /*
     *NB: We cannot directly iterate through a heterogeneous tuple using a for loop because its elements can have different types. To work with such a tuple, you'll need to access its elements individually.
     *
    for elem in &[tuple_1.0, tuple_1.1, tuple_1.2, tuple_1.3]  {
        println!("{}th elem: {}", loop_count, elem);
        loop_count += 1;
    }*/


    
    // Create a homogeneous tuple with same data type
    let tuple_2: (i8,u8,u8,i8,u8) = (-1,2,3,-4,5);
    println!("Tuple 2: {:?}", tuple_2);


    /*
    // Iterate through all the elems of tuple 2
    for elem in tuple_2 {
        println!("{}", elem);
    }*/

    // NB: For the time being, we cannot iterate over all the elems of tuple, since they consists
    // of heterogeneous data types.
}
