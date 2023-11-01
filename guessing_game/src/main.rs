use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game.");
    println!("Enter your guess (1 - 5):");

    let secret_number: u16 = rand::thread_rng().gen_range(1..=5); // inclusive range 1 through 5

    loop {
        let mut guess: String = String::new(); // create a mutable variable called guess and initialize it to an empty string

        io::stdin()
            .read_line(&mut guess) // we pass a reference to guess so it gets filled with user input
            .expect("Failed to read line.");

        let guess: u16 = match guess
            .trim()  // Remove all trailing whitespace characters
            .parse() // Attempt to convert string to integer
        {
            Ok(num) => num,     // if the input is a valid number convert
            Err(_) => continue, // if the input is an invalid number we return to the start of the loop
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You Win!");
                break; // if we win we break out of the loop
            }
        }
    }
}
