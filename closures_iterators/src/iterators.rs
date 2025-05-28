pub fn examples() {
    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
}

fn first_example() {
    let list = vec![1, 2, 3];

    let list_iter = list.iter();

    for val in list_iter {
        println!("Got: {val}");
    }
}

fn second_example() {
    let list = vec![10, 20, 30];

    let mut list_iter = list.iter();

    assert_eq!(list_iter.next(), Some(&10));
    assert_eq!(list_iter.next(), Some(&20));
    assert_eq!(list_iter.next(), Some(&30));
    assert_eq!(list_iter.next(), None);
}

fn third_example() {
    let list = vec![100, 200, 300];

    let list_iter = list.iter();

    let total: i32 = list_iter.sum();

    println!("{total}"); // 100 + 200 + 300 -> 600
}

fn fourth_example() {
    let list = vec![1000, 2000, 3000];

    let doubled_list: Vec<_> = list.iter().map(|el| el * 2).collect();

    println!("doubled_list = {:?}", doubled_list);
}

fn fifth_example() {
    #[derive(Debug, PartialEq)]
    struct Shoe {
        size: u8,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: "Sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: "Sandal".to_string(),
        },
        Shoe {
            size: 10,
            style: "Boot".to_string(),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("in_my_size = {:?}", in_my_size);
}
