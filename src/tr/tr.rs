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
struct Person {
    name: String
}

impl Person {
    // example of into
    // syntax 1:
    /* fn new<S>(name: S) -> Person where S: Into<String> { */
    // syntax 2:
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

fn into_demo() {
    let john = Person::new("John");

    // now say we have a String(not an &str) and want to pass this into Person
    // without having to explicitly convert the String into an &str
    // then in such cases we can use 'into'
    // into - allows automatic conversion where possible
    // see example above in impl Person
    let name: String = "Jane".to_string();
    let jane = Person::new(name);
}

// --------------------------------------------------
struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} has been created", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} has been destroyed", self.name);
    }
}

fn drop_demo() {
    let goblin = Creature::new("Jeff");
    println!("life moves on...");
    drop(goblin);

    // drop will automatically be called at end of scope even if not explicitly called
}

// --------------------------------------------------
use std::ops::{Add, AddAssign};

#[derive(Debug)]
struct Complex<T> {
    real: T,
    imag: T
}

impl<T> Complex<T> {
    fn new(real: T, imag:T) -> Complex<T> {
        // if you dont put :: here, compiler will treat < as a less-than operator
        Complex::<T> { real, imag }
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T>{
    // associated type - we have to specify the type of the output
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex { 
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

fn operator_overloading() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);

    /* println!("{:?}", a + b); */
    a += b;
    println!("{:?}", a)
}

// --------------------------------------------------
use std::mem;

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32 is = {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string is = {}", *self)
    }
}

fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

fn print_it_two(z: &Printable) {
    println!("{}", z.format());
}

fn static_dispatch() {
    let a = 123;
    let b = "hello".to_string();

    // static dispatch
    // ...at compile time, the implementation of Printable to be called (i.e. for i32 or for String) is decided by the value passed 
    // this is monomorphization
    // static disatch is faster
    print_it(a);
    print_it(b);
}

fn dynamic_dispatch() {
    let c = 456;
    let d = "world".to_string();

    // dynamic dispatch
    // we are just passing reference of values, so their type info is non-existent
    // ...at runtime, depending on the type of the value being referenced, the appropriate implementation is being called 
    // slower and takes up more memory
    // but dynamic dispatch is useful when we don't know thw type of the value being passed
    print_it_two(&c);
    print_it_two(&d);
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

    // DROP:
    drop_demo();

    // OPERATOR OVERLOADING:
    operator_overloading();

    // STATIC DISPATCH:
    static_dispatch();

    // DYNAMIC DISPATCH:
    dynamic_dispatch();
}