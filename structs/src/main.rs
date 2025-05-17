#![allow(dead_code)]

fn main() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let mut user1 = User {
        active: true,
        email: String::from("kastastin@gmail.com"),
        username: "kastastin".to_string(),
        sign_in_count: 1,
    };

    user1.active = false;

    println!("1: {:#?}", user1);

    let user2 = build_user("bob@gmail.com".to_string(), "bob".to_string());
    println!("1: {:#?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn second_example() {
    let user1 = User {
        active: true,
        email: String::from("bob@gmail.com"),
        username: String::from("bob"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        email: user1.email,
        username: String::from("builder"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        active: false,
        ..user2
    };

    println!("2: {:#?}", user3);
}

fn third_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);
