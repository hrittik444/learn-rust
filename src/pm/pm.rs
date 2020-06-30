#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    Rgbcolor(u8, u8, u8), // tuple
    Cmykcolor { cyan:u8, magenta:u8, yellow:u8, black:u8 } // struct
}

// take note of the return syntax here
fn how_many(x:i32) -> String {
    String::from(match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        // ..= is an 'inclusive' range
        // we can give this range a name, here z, and then we can operate on this range later
        z @ 9..=11 => "many",
        _ if(x % 2 == 0) => "even number of",
        _ => "a few"
    })
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} apples", x, how_many(x));
    }

    let point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        /* (x,y) => println!("({},{})", x, y) */
        (_,y) => println!("(something,{})", y),

        // these calues can also be passed by reference or mutable reference
        // eg:
        /* (ref mut x,0) => println!("y axis, x = {}", x) */
    }

    let c:Color = Color::Cmykcolor { cyan:12, magenta:34, yellow:56, black:255 };
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        // .. here indicates that we only care about the black value and we don't care abouth the other values
        Color::Cmykcolor { black:255,.. } => println!("black"),
        Color::Rgbcolor(r,g,b) => println!("RGB({},{},{})", r,g,b),
        _ => println!("unknown color")
    }
}

// generics allow us to not have to specify the exact type we are using for elements
// done using <T.>
struct Point<T> {
    x:T,
    y:T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

pub fn generics() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 1.2, y: 3.4 };
    let c = Point { x: 0, y: 4 };
    // or
    /* let a:Point<i32> = Point { x: 0, y: 0 }; */
    /* let b:Point<f64> = Point { x: 1.2, y: 3.4 }; */
    /* let c:Point<u16,i32> = Point { x: 0, y: 4 }; */

    let my_line = Line { start: a, end: c };
}