#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    first_example();
    second_example();
}

fn first_example() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn second_example() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!(
        "2: some_number = {:?}, absent_number = {:?}",
        some_number, absent_number
    );
}
