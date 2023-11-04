// we take a immutable reference to a string slice and return a reference to a immutable string slice
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert the string slice to a byte slice

    for (index, &item) in bytes.iter().enumerate() {
        // we obtain the iterator and call enumerate which returns a tuple of the index and a reference to the element
        if item == b' ' {
            // we check if the element is a white space and if so we return a slice starting from the first index to the index of the whitespace
            return &s[..index];
        }
    }

    // if no whitespaces were found we return a slice of the entire string
    &s[..] // we can just return &s too
}

fn main() {
    println!("{}", get_first_word("some random words")); // some
}
