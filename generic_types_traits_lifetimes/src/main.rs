use generic_types::generic_type_examples;

mod generic_types;

fn main() {
    first_example();
    second_example();
    third_example();

    generic_type_examples();
}

fn first_example() {
    let number_list: Vec<u8> = vec![15, 5, 10, 50, 25];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("1: largest = {largest}");
}

fn second_example() {
    let number_list = vec![15, 5, 10, 50, 25];

    let result = largest(&number_list);

    println!("2: result = {result}");

    fn largest(list: &[u8]) -> &u8 {
        let mut largest = &list[0];

        for number in list {
            if number > largest {
                largest = number;
            }
        }

        largest
    }
}

fn third_example() {}
