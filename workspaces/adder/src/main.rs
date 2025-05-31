use add_one;

fn main() {
    let num = 10;
    println!("{num} + 1 = {}", add_one::add_one(num));
}

// $ cd workspaces -> cargo new adder -> cargo build
// $ cargo new add_one --lib
