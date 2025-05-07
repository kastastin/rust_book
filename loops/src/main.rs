fn main() {
    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
    six_example();
}

fn first_example() {
    let mut counter = 0;
    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("1: result = {result}");
}

fn second_example() {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn third_example() {
    let mut number = 3;

    while number != 0 {
        println!("3: number = {number}");
        number -= 1;
    }
}

fn fourth_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("4: the value is {}", a[index]);
        index += 1;
    }
}

fn fifth_example() {
    let arr = [10, 20, 30];

    for element in arr {
        println!("5: the value is {element}");
    }
}

fn six_example() {
    for number in (1..3).rev() {
        println!("6: rev number = {number}");
    }

    for number in 1..=5 {
        println!("6: number = {number}");
    }

    for number in 1..=5 {
        let x = number as f32 * 0.1;
        println!("6: number = {x}");
    }
}
