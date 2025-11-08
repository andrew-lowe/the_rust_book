use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // inclusive lower/upper

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();
        // read_line has a Result value (enum with 2 variants: Ok and Err)
        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");

        // rust lets us shadow variables
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal=> {
                println!("You win!");
                break  
            }
        }   
    }
}
