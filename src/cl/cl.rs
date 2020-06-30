#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();

                // reading from input
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // push_str allows us to append to string
                        // we remove the return char entered by user at end of input and append it to the total entry
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                // checking for failed entry
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}
