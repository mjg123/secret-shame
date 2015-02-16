#![allow(unstable)]

use std::io;
use std::rand;
use std::cmp::Ordering;
    
fn main() {
    println!("Guess the number!");

    let answer = (rand::random::<u32>() % 100) + 1;
    println!("(shhh the secret number is {})", answer);

    loop {
        print!("Please input your guess: ");

        let parsed = io::stdin().read_line()
            .ok()
            .expect("Failed to read line")
            .trim()
            .parse::<u32>();
        
        let input = match parsed {
            Some(num) => num,
            None => {
                println!("That wasn't a number!");
                continue;
            }
        };
        
        //    println!("You guessed: {}", input);

        match cmp(input, answer){
            Ordering::Less => { println!("{} is too small!", input); },
            Ordering::Greater => { println!("{} is too big!", input); },
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }

    println!("Finished, bye xx");
}

fn cmp(a:u32, b:u32) -> Ordering {
    if a<b { Ordering::Less }
    else if b<a { Ordering::Greater }
    else { Ordering::Equal }
}
