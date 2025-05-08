fn main() {
    first_example();
    second_example();
    third_example();
    fourth_example();
}

fn first_example() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("1: s = {s}");
}

fn second_example() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("2: s = {s}"); // error: borrow of moved value: `s`

    let x = 10;
    copy_ownership(x);
}

fn takes_ownership(some_string: String) {
    println!("2: some_String = {some_string}");
}

fn copy_ownership(some_integer: i32) {
    println!("2: some_integer = {some_integer}");
}

fn third_example() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("3: s1 = {s1}");
    // println!("3: s2 = {s2}"); // error: borrow of moved value: `s2`
    println!("3: s3 = {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn fourth_example() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("4: the length of {s2} is {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
