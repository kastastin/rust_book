fn main() {
    first_example();
    second_example();
}

fn first_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("1: the length of {s1} = {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn second_example() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("2: s = {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
