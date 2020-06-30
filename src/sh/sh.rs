#![allow(dead_code)]
use std::mem;

// a structure is a custom datatype
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0
    }
}

// MEMORY ALLOCATION:
// stack = LIFO, fast, limited memory, usually or short term storage
// heap = a reference to a memory location where the data is actually stored, usually for long term storage
// Boxing allows us to allocate memory in heap to variables
pub fn stack_heap() {
    
    // stack allcoation i.e. normal variable assignment
    let p1 = origin();
    // heap allocation, p2 is a reference to a memory location
    let p2 = Box::new(origin());

    println!("size of p1 = {} bytes i.e. 8 bytes for x and 8 bytes for y", mem::size_of_val(&p1));
    println!("size of p2 = {} bytes i.e. 8 bytes for the entire memory address where x and y are stored, which is 64-bits in case of this PC so 8 bytes", mem::size_of_val(&p2));

    // accessing the actual value of the memory location referenced by p2
    // this is called Un-Boxing i.e. relocating values back onto the stack from the heap
    let p3 = *p2;
    println!("size of p3 = {} bytes i.e. similar to p1", mem::size_of_val(&p3));
    println!("p3 is {},{}", p3.x, p3.y);
} 



