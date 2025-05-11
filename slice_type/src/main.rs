fn main() {
    first_example();
}

fn first_example() {
    let s = String::from("hello world");
    let index = first_word_without_slice(&s);
    println!("1: index = {index}");

    let first_word = first_word_with_slice(&s);
    println!("1: first_word = {first_word}");
}

fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
