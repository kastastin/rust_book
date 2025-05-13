fn main() {
    first_example();
    second_example();
}

fn first_example() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("1: config_max (match) = {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("1: config_max (if let) = {max}");
    }
}

fn second_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("2: State quarter from {:?}", state);
    } else {
        count += 1
    }

    println!("2: count = {count}");
}
