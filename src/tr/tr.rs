#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Debug;

trait Animal {
    // a function that accepts a string
    fn create(name: &'static str) -> Self;

    // an instance function name, which returns a string
    fn name(&self) -> &'static str;

    fn talk(&self) {
        // default behaviour of talk
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello!", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow!", self.name());
    }
}

// --------------------------------------------------
// we are not manually writing the debug trait
// instead we are using std's Debug trait ans asking the compiler to derive it from there
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// takes a trait as a param
fn print_info(shape: impl Shape) {
    println!("area = {}", shape.area());
}

// takes a trait as a param
// we use + when the arg (here, shape) needs to be able to implement multiple traits
// there are three syntax ways for writing this

// syntax 1:
/* fn print_more_info(shape: impl Shape + Debug) { */
// syntax 2:
/* fn print_more_info<T: Shape + Debug>(shape: T) { */
// syntax 3:
fn print_more_info<T>(shape: T) where T: Shape + Debug {
    println!("{:?}", shape);
    println!("area = {}", shape.area());
}

fn trait_parameters() {
    let c = Circle { radius: 2.0 };
    /* print_info(c); */
    print_more_info(c);
    let s = Square { side: 6.0 };
}

// --------------------------------------------------
fn into_demo() {
    
}

// --------------------------------------------------
pub fn traits() {
    /* let h1 = Human { name:"John" }; */
    /* let h1 = Human::create("John"); */
    let h1:Human = Animal::create("John");
    h1.talk();

    /* let c1 = Cat { name:"Bitch" }; */
    let c1 = Cat::create("Bitch");
    c1.talk();

    // TRAIT PARAMETERS:
    // demoing functions that take traits as params
    trait_parameters();

    // INTO:
    into_demo();
}