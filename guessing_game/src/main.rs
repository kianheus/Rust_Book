use std::cmp::Ordering;
use std::io;

use rand::Rng;

/*
    A simple game where the program selects a random number between 0 and 100, and we guess it!
    Includes simple error handling and an amazing congratulatory message if you find the right number.
*/

fn main() {
    println!("Guess the number between 0 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}.", secret_number);


    loop{
        println!("Please input a guess:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue}
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Hurray! Hurray!");
                break;
            }
        }

    }



}
