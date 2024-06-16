use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // start..=end
    let secret = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Input guess: ");
    
        // create a mutable (changeable) variable
        // guess is bound to a new empty string
        let mut guess = String::new();
        
        io::stdin()
            // read line, store input in guess argument
            // references are default immut, need mut
            .read_line(&mut guess)
            // error catching, if .read_line returns "err"
            .expect("Failed to read line");
           
        // {} are placeholders, like \n
        let guess = guess.trim();  // remove newlines
        println!("You guessed: {guess}"); 
           
        // parse guess from str to u32, with error check
        // match is like a switch statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  // if parsed, ok returns num val
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        
        match guess.cmp(&secret) {
            Ordering::Less    => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal   => {  // win condition
                println!("You got it!");
                break;
            }
        }
    }
}
