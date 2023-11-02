use core::num;

fn infinity() {
    // A regular infinite loop
    loop {
        println!("again");
    }
}

fn main() {
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // For each loop
    for num in a {
        println!("{num}");
    }

    // inclusive for loop
    for n in 0..=5 {
        // 0,1,2,3,4,5
        println!("{}", a[n]);
    }

    // exnclusive for loop
    for n in 0..5 {
        // 0,1,2,3,4
        println!("{}", a[n]);
    }

    // reverse loop
    for n in (0..=5).rev() {
        println!("{}", a[n]);
    }

    // for each reverse
    for num in a.iter().rev() {
        println!("{num}");
    }

    // Loops can also return values
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}"); // 20

    // A conditional while loop
    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // You can also optionally assign labels to the loops
    let mut count = 0;
    'outer: loop {
        if count > 3 {
            break;
        }

        loop {
            println!("{count}");
            count += 1;
            if count > 3 {
                break 'outer;
            }
        }
    }
}
