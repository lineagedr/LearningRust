fn main() {
    const GIFTS: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle-doves",
        "three French hens",
        "four calling birds",
        "five golden rings (five golden rings)",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        let suffix: &str = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!(
            "On the {}{} day of Christmas\nMy true love sent to me\n{}",
            day + 1,
            suffix,
            GIFTS[day]
        );

        if day > 0 {
            for n in (0..day).rev() {
                if n == 0 {
                    print!("and ");
                }
                println!("{}", GIFTS[n]);
            }
        }

        println!("\n");
    }
}
