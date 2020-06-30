#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

fn print_value(x: i32) {
    println!("value = {}", x);
}

// the function borrows the value (i.e. a reference to it), changes it and gives it back
fn change_value(x: &mut i32) {
    // de-reference the reference
    // i.e. accessing the value referenced by the reference
    *x += 1;
}

// -> and then specify the return type
fn return_value(x: i32, y: i32) -> i32 {
    // remember: no colon when we wanna return
    x * y
}

// FUNCTIONS:
pub fn functions() {
    print_value(12);

    let mut z = 1;
    change_value(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = return_value(a, b);
    println!("product = {}", p);
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    // a method must hae a reference to itself
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx*dx + dy*dy).sqrt()
    }
}

// METHODS:
pub fn methods() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let l1 = Line { start: p1, end:p2 };

    println!("length of line l1 = {}", l1.len());
}

fn say_hello() { println!("hello"); } 

// CLOSURES:
// storing a function in a variable
pub fn closures() {
    let sh = say_hello;
    sh();

    // one way of declaring closures = passing by value
    // parameters of an inline closure are represented by | |
    let plus_one = |x:i32| -> i32 {
        x + 1
    };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    // another way of declaring closures = passing by reference
    // here we are not specifying the type of the arg, neither are we specifying the return type
    // we will let the compiler figure these out for itself
    let plus_two = |x| {
        let z = x + 2;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    // if closures are defined withing a scope
    // then it gets destroyed after execution and we can roceed normally i.e. borrows are released and hence can be reused
    // eg:
    let mut three = 3;
    {
        let plus_three = |x| {
            let mut z = x;
            z += three;
            z
        };
        println!("{} + 3 = {}", 3, plus_three(3));
    }
    let borrow_three = &mut three;

    // yet another way of declaring closures = passing by mutable reference
    // not using curly braces here since function has only one statement
    let plus_four = |x:&mut i32| *x += 4;

    let mut f = 12;
    plus_four(&mut f);
    println!("f = {}", f);
}

fn is_even(x:u32) -> bool {
    x % 2 == 0
}

// HOC:
// functions that take functions as args and/or return functions

// below is a HOC
// this is a an example of an HOC returning a function
// notice the syntax
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // move - converts any variables captured by reference or mutable reference to owned by value variables
    move |y| {
        y > limit
    }
}

pub fn hocs() {
    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit);

    for i in 0.. {
        let i_square = i*i;
        if above_limit(i_square) {
            break;
        } else if is_even(i_square) {
            sum += i_square;
        }
    }
    println!("sum1 = {}", sum);

    // again sum of all even squares < 500
    // notice the use of Rust's built-in functions here
    // inside the fold, we have an example of an HOC taking a function
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|&x| is_even(x))
        .fold(0, |sum, x| sum + x);
    
        println!("sum2 = {}", sum2);
}