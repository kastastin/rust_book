use std::fmt::Display;

pub fn lifetime_examples() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let string1 = "abcd".to_string();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

fn second_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = "Call me Bob. Some years ago.".to_string();
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i = {:?}", i);
}

fn third_example() {
    let first = "ab".to_string();
    let second = "xyz".to_string();
    let attention = "Attention!".to_string();

    let result = longest_with_an_announcement(&first, &second, attention);

    println!("result = {result}");

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);

        if x.len() > y.len() { x } else { y }
    }
}
