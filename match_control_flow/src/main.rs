fn main() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let a = value_in_cents(Coin::Penny);
    let b = value_in_cents(Coin::Nickel);
    let c = value_in_cents(Coin::Dime);
    let d = value_in_cents(Coin::Quarter);

    println!("1: a = {a}, b = {b}, c = {c}, d = {d}");

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn second_example() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("2: six = {:?}, none = {:?}", six, none);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}

fn third_example() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {
        println!("3: add_fancy_hat");
    }

    fn remove_fancy_hat() {
        println!("3: remove_fancy_hat");
    }

    fn move_player(num_spaces: u8) {
        println!("3: num_spaces = {num_spaces}");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
