/*
    Ownership Rules:

    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.
*/

fn main() {
    // variable s is a string literal the value hello
    // is hardcoded into the text of our program
    // the value is valid from the point at which it's declared
    // until the end of the current scope
    let s: &str = "hello";

    // With the String type, in order to support a mutable, growable piece of text,
    // we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    // That first part is done by us: when we call String::from, its implementation requests the memory it needs.
    // This is pretty much universal in programming languages.
    // Also the memory is returned once the variable that owns it goes out of scope
    let mut s: String = String::from("Hello");

    // append a string literal to our string
    s.push_str(", World");

    // We're creating a new variable called new_str in Rust we don't do deep copies but shallow copies and also
    // s is now invalidated and we can't re-use the variable s because we've moved it
    let new_str: String = s;

    // if we want to make a deep copy we can use the .clone method
    let mut deep_copy = new_str.clone();

    // we can still use new_str because we've only made a copy of it
    // we haven't moved it
    println!("{new_str}"); // Hello, World

    // we can create a mutable reference with &mut keyword
    let deep_ref = &mut deep_copy;

    // modifying deep_ref will also modify deep_copy since it's a mutable reference to deep_copy
    deep_ref.push_str("!");

    println!("{deep_copy}"); // Hello, World!

    modify_val(&mut deep_copy); // we pass a mutable reference to modify_val

    println!("{deep_copy}"); // Hello, World!!
}

fn modify_val(s: &mut String) {
    s.push('!');
}
