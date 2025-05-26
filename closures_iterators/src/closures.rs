use std::{thread, time::Duration};

pub fn examples() {
    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
}

fn first_example() {
    let expensive_closure = |num: i8| -> i8 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = expensive_closure(100);

    println!("x = {x}");
}

// NOTES:
// fn  add_one_v1   (x: i32) -> i32 { x + 1 }
// let add_one_v2 = |x: i32| -> i32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

fn second_example() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);

    only_borrows();

    println!("After calling closure: {:?}", list);
}

fn third_example() {
    let mut list = vec![10, 20, 30];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(40);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn fourth_example() {
    let list = vec![100, 200, 300];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn fifth_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u8,
        heigh: u8,
    }

    let mut list = [
        Rectangle {
            width: 10,
            heigh: 2,
        },
        Rectangle { width: 4, heigh: 6 },
        Rectangle { width: 8, heigh: 4 },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations}", list);
}
