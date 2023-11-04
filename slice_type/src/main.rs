/*
    The Slice Type
    Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
*/

fn main() {
    let s: String = String::from("Hello World!");

    let hello_slice = &s[0..5]; // Hello

    // With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
    // let hello_slice = &s[..5]; // Hello

    // You can also drop both values to take a slice of the entire string. So these are equal:
    // let hello_slice = &s[..]; // Hello World!

    let world_slice = &s[6..s.len()]; // World!

    println!("{hello_slice} {world_slice}"); // Hello World!

    // we can also use slicing for arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a_slice = &a[..3]; // 1, 2, 3

    for n in a_slice {
        println!("{n}") // 1, 2, 3
    }
}
