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

        let guess: u16 = guess
            .trim() // remove all whitespaces
            .parse() // convert from string to u16
            .expect("Please type a number!");

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
