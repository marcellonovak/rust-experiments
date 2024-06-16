use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's play guess the number!");
    
    // start..=end rand format
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Input guess: ");
    
    loop {
    
        // create a mutable (changeable) variable
        // guess is bound to a new empty string
        let mut guess = String::new();
        
        io::stdin()
            // read line, store input in guess argument
            // references are default immut, need mut
            .read_line(&mut guess)
            // error catching if .read_line returns "err"
            .expect("Failed to read line");
            
        // parse guess from str to u32, with error check
        let guess: u32 = guess.trim().parse().expect("Non-number!");
        
        // {} are placeholders, like \n
        println!("You guessed: {guess}");

        // kinda like a switch case? comparison
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
