#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;

use std::thread;
use std::time;

// VECTORS:
// vectors are similar to arrays, but can have dynamic size
fn vectors() {
    let mut a = Vec::new();

    // adding elements to an array
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);

    let index:usize = 1;
    println!("a[1] = {}", a[index]);

    a[index] = 4;
    println!("a[1] = {}", a[index]);

    // get can be used to get a value at a particular index
    // it returns an Option
    // here let's try to get the 6th element in a vector of only 4 elements
    // since we are using get, it won't crash the program
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error: no such element")
    }

    // iterating over a vector
    for x in &a {
        println!("{}", x);
    }

    // adding elements to vector
    a.push(4);

    // iterating over a vector using debug output
    println!("{:?}", a);

    // removing elements from vector
    // using pop
    // pop retrns an Option; Some contains the popped element and None will have error if any (eg: if vector was empty to begin with)
    let last_element = a.pop();
    println!("popped element = {:?}, a = {:?}", last_element, a);

    // here we are printing a vector in reverse order
    // this is a good example of iterating over something that returns an option
    while let Some(x) = a.pop() 
    // i.e. while popping an element gives a Some and not a None
    {
        println!("{}", x);
    }
}

// HASH MAPS:
// containers for pairs of values
// you can use the first element of the pair to quickly find the second element of that pair
fn hash_maps() {
    let mut shapes = HashMap::new();

    // adding elements to hash map
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    // we are accessing the value corresponding to the key "square" in the hash map
    // here, into converts this value into a string
    println!("a square has {} sides", shapes["square".into()]);

    // iterating over a hash map
    for (key, value) in &shapes {
        println!("{} = {}", key, value);
    }

    // modifying values in hash map
    shapes.insert(String::from("square"), 5);
    println!("{:?}", shapes);

    // .entry and .or_insert will only insert an element if that key doesn't already exist
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);

    // .entry also allows us to modify the value of that key does exist
    // in the below example we are borrowing the entire hashmap as a mutable, so defining a separate scope for that
    {
        // this gives us a reference to the value corresponding to the key "circle" in the hash map
        // btw this .or_inser here is redundant and has no meaning
        let actual = shapes.entry("circle".into()).or_insert(1);

        // now we are modifying the value of "circle"
        *actual = 0;
    }
    println!("{:?}", shapes);

    // but if we try to access a key that doesn't exist, the program will crash
    // so we can handle such cases with
}

// HASH SETS:
// a collection of unique values stored in a container
// unordered
fn hash_sets() {
    let mut greek = HashSet::new();

    // adding elements to hash set
    greek.insert("alpha");
    greek.insert("beta");
    println!("{:?}", greek);

    // if we try to enter duplicate elements, nothing will happen
    greek.insert("beta");
    println!("{:?}", greek);

    // is_added will be true if the insert was successful and false if not
    let is_added = greek.insert("gamma");
    if is_added { println!("added successfully"); } else { println!("nope"); }
    let is_added = greek.insert("gamma");
    if is_added { println!("added successfully"); } else { println!("nope"); }

    // .contains returns a boolean depending on whether the element is there
    if !greek.contains("delta") { println!("delta not found"); }

    // removing elements from hash set\
    // here, this boolean value is stored in is_removed 
    let is_removed = greek.remove("beta");

    if is_removed { println!("removed beta"); }
    println!("{:?}", greek);
}

pub fn standard_collection() {
    vectors();
    hash_maps();
    hash_sets();
}