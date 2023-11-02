use std::io;

fn process_user_input(choice: bool) {
    loop {
        println!(
            "Enter a valid temperature in {}",
            if choice == true {
                "Celsius"
            } else {
                "Fahrenheit"
            }
        );

        let mut input: String = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // Convert string to i32
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == true {
            // (°C × 9/5) + 32
            println!("{input}°C -> {}°F", ((input * 9 / 5) + 32));
        } else {
            // (°F − 32) × 5/9
            println!("{input}°F -> {}°C", (input - 32) * 5 / 9);
        }

        break;
    }
}

fn main() {
    loop {
        println!("1. Celsius to Fahrenheit.");
        println!("2. Fahrenheit to Celsius.");
        println!("3. Exit.");

        let mut choice: String = String::new();

        // Read user choice
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        // Convert string to i32
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            // C to F
            1 => process_user_input(true),
            // F to C
            2 => process_user_input(false),
            3 => break,
            _ => {
                println!("Invalid choice. Please select 1 or 2.");
            }
        }
    }
}
