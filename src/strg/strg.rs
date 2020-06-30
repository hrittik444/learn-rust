#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub fn strings() {
    // strings can be of two types
    // &*static str = string slice
    // string slices cannot be reassigned i.e. the string cannot be mutated or manipulated
    // it is a valid utf-8 sequence
    let s = "hello there";
    // or
    /* let s:&*static str = "hello there"" */

    // printing all chars in string
    for c in s.chars() {
        println!("{}", c);
    }

    // printing all chars in string in reverse order
    for c in s.chars().rev() {
        println!("{}", c);
    }

    // accessing individual characters in a string
    // .chars() returns an option
    if let Some(first_char) = s.chars().nth(0) {
        println!("first char of the string = {}", first_char);
    }

    // another type of string is
    // the String construct
    // String is a heap allocation
    // it is a valid utf-8 sequence
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(","); // not push but push_str
        a += 1;
    }
    println!("letters: {}", letters);

    // conversion of one type of string to the other
    // i.e. &str <--> String
    let u:&str = &letters;
    println!("u = {}",u);

    // concatenation
    let letters1 = letters + "abc";
    println!("letters1 = {}", letters1);

    // initializing strings
    let abc = String::from("yo mama ");
    // or
    let def = "so fat".to_string();
    println!("abc = {}, def = {}", abc, def);

    // more concatenation
    println!("{:?}", abc + &def);

    // some more concatenation
    let mut ghi = "".to_string();
    let jkl = "yo mama ".to_string();
    let mno = "so fat".to_string();
    ghi.push_str(&jkl);
    ghi.push_str(&mno);
    println!("{}", ghi);

    // manipulation
    let mut pqr = "hello world".to_string();
    pqr.remove(0);
    pqr.push_str("!!!");
    println!("{}", pqr.replace("ello", "goodbye"));

    // the FORMAT macro for string formatting
    format_macro();
}

fn format_macro() {
    let name = "Hrittik";
    let addressed_to = "World";
    let greeting = format!("hi {}, I'm {}, nice to meet you", addressed_to, name);
    println!("{}", greeting);

    let run = "run";
    let forrest = "Forrest";
    let rfr = format!("{0}, {1}, {0}", run, forrest);
    println!("{}", rfr);

    let info = format!("the name's {last}, {first} {last}.", first = "James", last = "Bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {} {data}", 
        "alpha", "beta", data = "delta"
    );
    println!("{}", mixed);
}