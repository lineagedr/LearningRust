/*
    The Array Type
    Another way to have a collection of multiple values is with an
    array. Unlike a tuple, every element of an array must have the
    same type. Unlike arrays in some other languages, arrays in
    Rust have a fixed length.

    The array is also stored on the stack.
*/

fn main() {
    // we can declare an array like so
    let arr = [1, 2, 3, 4, 5];

    // or if we want to be more specific like so
    let other_arr: [i32; 5] = [5, 3, 2, 66, 1];

    // You can also initialize an array to contain the same value for
    // each element by specifying the initial value, followed by a
    // semicolon, and then the length of the array in square brackets,
    // as shown here:
    let a = [3; 5]; // equvalent to a = [3, 3, 3, 3, 3];

    // or if we want to be moe specific like so
    let a: [u32; 5] = [3; 5];

    // evaluates to let months: [&str; 12]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // we can also iterate over the values using a loop
    for month in months {
        println!("{month}");
    }

    println!("{}", months[20]);
}
