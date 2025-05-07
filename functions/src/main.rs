fn main() {
    another_function(10);

    let x = plus_one(100);
    println!("x = {x}");
}

fn another_function(x: i32) {
    println!("Another function: x - {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
