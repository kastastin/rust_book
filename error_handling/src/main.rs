use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[10]; // index out of bounds: the len is 3 but the index is 10

    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
    sixth_example();
    seventh_example();
}

fn first_example() {
    let first_file_result = File::open("first.txt");

    let first_file = match first_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("first.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("1: first_file = {:?}", first_file);
}

fn second_example() {
    let second_file = File::open("second.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("second.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("2: second_file = {:?}", second_file);

    // let text_file = File::open("text.txt").expect("text.txt should be included in this project");
    // println!("2: text_file = {:?}", text_file); // panic
}

fn third_example() {
    let username = read_username_from_file();

    println!("3: username = {:?}", username);

    fn read_username_from_file() -> Result<String, io::Error> {
        let third_file_result = File::open("third.txt");

        let mut username_file = match third_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
}

fn fourth_example() {
    let username = read_username_from_file();

    println!("4: username = {:?}", username);

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut fourth_file = File::open("fourth.txt")?;

        let mut username = String::new();

        fourth_file.read_to_string(&mut username)?;

        Ok(username)
    }
}

fn fifth_example() {
    let username = read_username_from_file();

    println!("5: username = {:?}", username);

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("fifth.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
}

fn sixth_example() {
    let username = read_username_from_file();

    println!("6: username = {:?}", username);

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("sixth.txt")
    }
}

fn seventh_example() {
    let last_char1 = last_char_of_first_line("lol"); // Some('l')
    let last_char2 = last_char_of_first_line(""); // None

    println!("7: last_char1 = {:?}", last_char1);
    println!("7: last_char2 = {:?}", last_char2);

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
