// Tut-17.3: Trait & Struct

#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle { length: f64, width: f64 }
#[derive(Debug)]
struct Circle { length: f64, width: f64 }

trait Shape {
    fn description(length: f64, width: f64) -> Self;
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn description(length: f64, width: f64) -> Self { 
        // Rectangle { length, width }  // OK-1
        Self { length, width }  // OK-2
    }

    fn area(&self) -> f64 {
        self.length * self.width
    }
}

const PI: f64 = 3.1416;

impl Shape for Circle {
    fn description(length: f64, width: f64) -> Self {
        // Circle { length, width }     // OK-1
        Self { length, width }      // OK-2
    }

    fn area(&self) -> f64 {
        (self.length / 2.0).powf(2.0) * PI
    }
}

fn main() {
    let rec = Rectangle { length: 10.0, width: 10.0 };
    let circ = Circle { length: 10.0, width: 10.0 };

    // println!("rec length: {}", rec.length);
    println!("Rectangle: {:?}", Rectangle::description(rec.length, rec.width));
    println!("Rectangle's area: {}", Rectangle::area(&rec));

    println!("Circle: {:?}", Circle::description(circ.length, circ.width));
    println!("Circle's area: {:?}", Circle::area(&circ));
}