#![allow(dead_code)]

use std::mem;

// STRUCTS:
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

// ENUMS:
enum Color {
    Red,
    Green,
    Blue,
    Rgbcolor(u8, u8, u8), // tuple
    Cmykcolor { cyan:u8, magenta:u8, yellow:u8, black:u8 } // struct
}

// UNIONS:
// below union stores either an integer or a floating point number
// unions are mainly used for interoperability with C/C++
union IntOrFloat{
    i: i32,
    f: f32
}

pub fn data_str() {
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("p has x = {} and y = {}", p1.x, p1.y);

    // pattern matching using enums
    pattern_match_enum();

    let mut iof = IntOrFloat{ i:123 };
    iof.i = 234;

    // pattern matching using unions
    pattern_match_union(IntOrFloat { f:42.0 });

    // must put unsafe since we don't know if the value is an integer or float
    let value = unsafe { iof.i };
    println!("iof = {}", value);

    // OPTION<T>:
    // an option returns two possible values
        // if operation was successful, it returns Some(v) where v is the value being returned
        // if operation was unsuccessful, it returns None
    let x = 3.0;
    let y = 0.2;

    let result = 
        if y != 0.0 {
            Some(x/y)
        } else {
            None
        };

    // pattern matching on option
    match result {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("cannot divide by zero")
    }

    // another way of pattern matching on option
    if let Some(z) = result {
        println!("{}/{} = {}", x,y,z);
    }

    // another way of pattern matching on option
    /* while let Some(z) = result {
        println!("{}/{} = {}", x,y,z);
    } */

    // ARRAYS:
    arrays();

    // SLICES
    slices();

    // TUPLES
    // tuples are several values taken together as a collection
    // (represented with paranthesis)
    // an array can have elements of the same type
    // tuple can have elements of different types
    tuples();
}

fn pattern_match_enum() {
    // here c is an instance of the Color enum
    /* let c:Color = Color::Blue; */
    /* let c:Color = Color::Rgbcolor(5,6,7); */
    let c:Color = Color::Cmykcolor { cyan:12, magenta:34, yellow:56, black:25 };

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::Rgbcolor(0,0,0)
            | Color::Cmykcolor { cyan:_, magenta:_, yellow:_, black:255 } => println!("black"),
        Color::Rgbcolor(r,g,b) => println!("RGB({},{},{})", r,g,b),
        _ => println!("unknown color")
    }
}

fn pattern_match_union(iof: IntOrFloat) { 
    unsafe {
        match iof {
            IntOrFloat { i:42 } => {
                println!("value is 42");
            },

            // if value is not integer 42, then assume value is a floating point number and treat it as such
            IntOrFloat { f } => {
                println!("value is {}", f);
            }
        }
    }
}

fn arrays() {
    // array_one has 5 32-bit integers
    // we need to know how many elements the array will have
    let mut array_one:[i32; 5] = [1,2,3,4,5];
    // or
    /* let mut array_one = [1,2,3,4,5]; */

    array_one[0] = 678;
    println!("array_one has {} elements, first element is {}",
        array_one.len(), array_one[0]
    );

    // instead of using a loop to print out the whole array
    // we can use the debug output syntax
    println!("{:?}", array_one);

    if array_one != [1,2,3,4,5] { 
        println!("array has been mutated");
    }

    // bulk-filling an array
    let array_two = [1; 10];
    for i in 0..array_two.len() {
        println!("{}", array_two[i]);
    }

    println!("array_two takes up {} bytes", mem::size_of_val(&array_two));

    let array_three = [1u16; 10];
    println!("array_three takes up {} bytes", mem::size_of_val(&array_three));

    // multi-dementional array
    let array_four:[[f32; 3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", array_four);

    for i in 0..array_four.len() {
        for j in 0..array_four.len() {
            if i == j {
                println!("array_four[{}][{}] = {}", i,j,array_four[i][j]);
            }
        }
    }

    // remember: arrays cannot be resized
}

// the & means we are taking a slice (i.e. borrowing) a part of an array of i32 elements
fn use_slice1(slice1: &[i32]) {
    println!("first element = {}, length = {}", slice1[0],slice1.len());
}

// bowworing the whole array and being able to mutate it
fn use_slice2(slice2: &mut[i32]) {
    println!("first element = {}, length = {}", slice2[0],slice2.len());
    slice2[0] = 123456;
}

fn slices() {
    // unlike an array, the actual size of a slice is not known at compile time
    let data1 = [1,2,3,4,5];

    // elements at pos 1 to 3 will be passed to use_slice
    use_slice1(&data1[1..4]);

    let mut data2 = [6,7,8,9,0];
    /* use_slice2(&mut data2[1..4]); */
    use_slice2(&mut data2);

    // array has been mutated by function use_slice2 which borrowed the array
    println!("{:?}", data2)
}

// this function returns a tuple
fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples() {
    let x = 5;
    let y = 6;
    let snp = sum_and_product(x,y);
    println!("snp = {:?}", snp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, snp.0, snp.1);

    // destructuring a tuple
    let (a, b) = snp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let snp2 = sum_and_product(7, 8);
    let snp3 = (snp, snp2);
    println!("snp3 = {:?}", snp3);
    println!("last element of snp3 = {}", (snp3.1).1);

    // tuple can contain values of different types
    let new_tuple = (true, 45.0, -1i16);
    println!("new_tuple = {:?}", new_tuple);

    // making a tuple containing a simgle element
    // (45) wont work as it will be treated as an integer, so use (45,)
    let single_tuple = (45,);
    println!("single_tuple = {:?}", single_tuple);
}
