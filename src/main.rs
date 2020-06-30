use std::mem;

// importing modules from other files
#[path = "cf/cf.rs"]
mod cf;
#[path = "sh/sh.rs"]
mod sh;
#[path = "cl/cl.rs"]
mod cl;
#[path = "ds/ds.rs"]
mod ds;
#[path = "pm/pm.rs"]
mod pm;
#[path = "st/st.rs"]
mod st;
#[path = "strg/strg.rs"]
mod strg;
#[path = "ng/ng.rs"]
mod ng;
#[path = "fnc/fnc.rs"]
mod fnc;
#[path = "tr/tr.rs"]
mod tr;

// GLOBAL VARIABLES:
// can be declared using const or statis
// type has to be specified
// static mutables are unsafe
const CONST_ONE: u8 = 5;
static CONST_TWO: i32 = -5;
static mut CONST_THREE: i32 = -10;

fn main() {
    // VARIABLES:
    let a: u8 = 123;
    println!("a = {}", a);
    // a = 234;

    let mut b: i8 = 0;
    println!("b = {}", b);
    b = -23;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -123;
    println!("c = {}", c);
    
    // we need isize if the variable's size needs to be same as size of mem addr of the OS, here 64-bit
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, size = {} bytes, on a {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let f: f32 = 2.5;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    // OPERATORS:
    let mut h = 12;
    h += 5;
    println!("h = {}", h);
    println!("remainder of {} / {} is {}", h, 3, h % 3);

    let h_cubed = i32::pow(h, 3);
    println!("{} cubed = {}", h, h_cubed);

    let i = 2.5;
    let i_to_power_three = f64::powi(i, 3);
    let i_to_power_pi = f64::powf(i, std::f64::consts::PI);
    println!(
        "{} cubed = {}, to the power of pi = {}",
        i, i_to_power_three, i_to_power_pi
    );

    // bitwise operators
    // | is OR, & is AND, ^ is XOR, ! is NOR
    let c = 1 | 2;
    println!("1 | 2 = {}", c);

    //bitwise shift
    let two_to_power_ten = 1 << 10;
    println!("2 to power of 10 = {}", two_to_power_ten);

    let x = 1 < 2;
    let y = 34 >= 22;
    let z = 55 == 55;
    println!("1<2 = {}, 34 >= 22 = {}, 55 == 55 = {}", x, y, z);

    // SCOPE AND SHADOWING:
    scope_and_shadowing();

    // CONSTANTS AND GLOBAL VARIABLES:
    println!("CONST_ONE = {}", CONST_ONE);
    println!("CONST_TWO = {}", CONST_TWO);

    // remember static mutables are unsafe
    unsafe {
        CONST_THREE = -20;
        println!("CONST_THREE = {}", CONST_THREE);
    }

    // MEMORY ALLOCATION:
    // execute function from imported module
    sh::stack_heap();

    // CONTROL FLOW:
    cf::if_statememt();

    // COMBINATION LOCK GAME:
    /* cl::combination_lock(); */

    // DATA STRUCTURES:
    ds::data_str();

    // PATTERN MATCHING and GENERICS:
    pm::pattern_matching();
    pm::generics();

    // STARNDARD COLLECTIONS:
    st::standard_collection();

    // STRINGS:
    strg::strings();

    // NUMBERS GUESSING GAME:
    /* ng::numbers_game(); */

    // FUNCTIONS:
    fnc::functions();

    // MEHODS:
    fnc::methods();

    // CLOSURES:
    fnc::closures();

    // HIGHER ORDER FUNCTIONS:
    fnc::hocs();

    // TRAITS:
    tr::traits();
}

fn scope_and_shadowing() {
    let a = 0;
    println!("{}", a);

    {
        let b = 1;
        let a = 2;
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
    }
    println!("outside, a = {}", a);
}
