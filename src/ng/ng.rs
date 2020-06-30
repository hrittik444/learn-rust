#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rand::Rng;
use std::io::stdin;

pub fn numbers_game() {
    // exclusive range
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("enter your guess: ");

        let mut choice = String::new();

        match stdin().read_line(&mut choice) {
            Ok(_) => {
                // trim_end trims the \n at the end of the input
                // parse::<i64>() since we will parse the input as an i64 number
                let parsed = choice.trim_end().parse::<i64>();

                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("out of range");
                        } else if guess < number {
                            println!("too low");
                        } else if guess > number {
                            println!("too high");
                        } else {
                            println!("correct!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}