const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x: {x}");

    x = 6;
    println!("The value of x: {x}");

    println!("{THREE_HOURS_IN_SECONDS}");

    let a = 10;

    let a = a + 1;

    {
        let a = a * a;
        println!("The value of 'a' is [Inner Scope]: {a}");
    }

    println!("The value of 'a' is [Outer Scope]: {a}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);

    let (id, name) = get_user();
    println!("ID: {id}, Name: {name}");

    let arr = [0; 4]; // [0, 0, 0, 0]
    println!("Array: {:?}", arr);
}

fn get_user() -> (u32, String) {
    (432, String::from("Konstantin"))
}
