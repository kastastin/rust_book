pub fn string_examples() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let s1 = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    println!(
        "------------\n\
        Strings - 1:\n\
        s1 = {s1}\n\
        s2 = {s2}\n\
        s3 = {s3}\n\
        ------------"
    );
}

fn second_example() {
    let mut s = "lo".to_string();
    s.push_str("l");

    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let tic = "tic".to_string();
    let tac = "tac".to_string();
    let toe = "toe".to_string();

    let tic_tac_toe = format!("{tic}_{tac}_{toe}");

    println!(
        "Strings - 2:\n\
        s = {s}\n\
        s3 = {s3}\n\
        tic_tac_toe = {tic_tac_toe}\n\
        ------------"
    );
}

fn third_example() {
    println!("Strings - 3:");

    for c in "lol".chars() {
        println!("{c}");
    }

    println!("------------");
}
