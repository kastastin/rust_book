fn main() {
    first_example();
    second_example();
    third_example();
    fourth_example();
}

fn first_example() {
    let number = 3;

    if number < 5 {
        println!("1: Condition was true");
    } else {
        println!("1: Condition was false");
    }
}

fn second_example() {
    let number = 10;

    if number != 10 {
        println!("2: number was smth other than zero");
    }
}

fn third_example() {
    let number = 6;

    if number % 4 == 0 {
        println!("3: number is divisible by 4");
    } else if number % 3 == 0 {
        println!("3: number is divisible by 3");
    } else if number % 2 == 0 {
        println!("3: number is divisible by 2");
    } else {
        println!("3: number is not divisible by 4, 3 or 2");
    }
}

fn fourth_example() {
    let condition = true;

    let number = if condition { 5 } else { -5 };
    println!("4: number: {number}");
}
