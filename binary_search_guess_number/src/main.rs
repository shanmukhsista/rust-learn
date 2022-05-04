use rand::{self, Rng};
use std::io;

/**
 * This program tries to generate a random number and makes the user guess the number.
 * It leverages binary search algorithm and outputs the number of steps to
 * highlight the computational complexity.
 */

fn main() {
    let max_range = 1_000_001;
    // compute the max steps it takes to guess the number.
    let max_steps: u32 = (max_range as f32).log(2.0).ceil() as u32;
    println!("Enter a number between 1 and 1,000,000 ( 1 Million ) ");

    let mut num_steps = 0;

    // initialize a variable to guess.
    let random_number: u32 = rand::thread_rng().gen_range(0..max_range);
    loop {
        let mut user_guess = String::new();
        // Run a loop until user guesses the number.
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Unable to parse input");
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        match user_guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Guessed Number is smaller than the target "),
            std::cmp::Ordering::Equal => {
                println!("You've got this. Guessed correct number in {} attempts . \n Computational complexity should always give a result in less than {} guesses. " , num_steps, max_steps );
                // Exit loop if everything works out.
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Guessed number is greater than target. Please try again. ");
            }
        }
        num_steps = num_steps + 1;
    }
}
