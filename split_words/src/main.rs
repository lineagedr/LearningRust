fn split_words(s: &str, c: char) {
    let words: Vec<&str> = s.split_terminator(c).collect();

    for word in words {
        println!("{word}");
    }
}

fn main() {
    split_words("some words go here and are seperated by space", ' ');
}
