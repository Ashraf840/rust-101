// Tut-0.1: Avoid raising warning from unused variable


fn main() {
    // To avoid raising error for any unused variable (which might be used later), then use an
    // underscore as a prefix.
    // let is_true: bool = true;    // It'll throw a warning & won't compile code
    let _is_true: bool = true;  // It'll prevent the warning & compile the code
}
