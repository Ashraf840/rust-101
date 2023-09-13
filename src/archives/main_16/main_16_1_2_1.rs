#![allow(dead_code)]

#[derive(Debug)]
pub struct StructOne {
    pub some_bool: bool,
    pub some_int: i8,
}


impl StructOne {
    pub fn is_smaller(&self, val: i8) -> bool {
        self.some_int < val
    }
}