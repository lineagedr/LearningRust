/*
    The Tuple Type
    A tuple is a general way of grouping together a number of
    values with a variety of types into one compound type. Tuples
    have a fixed length: once declared, they cannot grow or shrink
    in size.
*/

fn main() {
    let tup: (i32, f32) = (100, 5.4);

    // we can access the tuples elements using the . starting from 0
    println!("{}, {}", tup.0, tup.1);

    // we can also assign variables to tuples like so
    // also this is called "destructuring"
    let (x, y) = tup;

    println!("{x}, {y}");

    // we can also print all tuples
    let other: (i32, f32, u8, i128) = (100, 13.4, 255, 9000);
    println!("{:?}", other);
}
