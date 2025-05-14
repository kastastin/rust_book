use std::collections::HashMap;

pub fn hashmap_examples() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let mut scores: HashMap<String, u8> = HashMap::new();

    scores.insert("white".to_string(), 10);
    scores.insert("black".to_string(), 5);

    // let team_name: Option<&u8> = scores.get("white");
    // let team_name: Option<&u8> = scores.get("no"); // None
    // let team_name = scores.get("no").copied().unwrap_or(0); // 0
    let team_name = scores.get("white").copied().unwrap_or(0);

    println!(
        "Hash Maps - 1:\n\
      scores = {:?}\n\
      team_name = {:?}",
        scores, team_name
    );

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn second_example() {
    let mut scores = HashMap::new();

    scores.insert("black".to_string(), 1);
    scores.insert("black".to_string(), 100); // {"black": 100} -> the original value 1 has been overwritten

    scores.entry("white".to_string()).or_insert(0); // {"black": 100, "white": 0}
    scores.entry("black".to_string()).or_insert(333); // {"white": 0, "black": 100}

    println!(
        "------------\n\
      Hash Maps - 2:\n\
      scores = {:?}\n\
      ------------",
        scores
    );
}

fn third_example() {
    let text = "red blue red green";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!(
        "------------\n\
    Hash Maps - 3:\n\
    map = {:?}\n\
    ------------",
        map
    );
}
