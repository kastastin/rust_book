fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "1: The area of the rectangle is {} square pixels.",
        area_first_example(width1, height1)
    );

    let rect2 = (30, 50);

    println!(
        "2: The area of the rectangle is {} square pixels.",
        area_second_example(rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "3: The area of the rectangle is {} square pixels.",
        area_third_examle(&rect3)
    );

    println!("3: rect3 is {:?}", rect3);

    dbg_example();
}

fn area_first_example(width: u32, height: u32) -> u32 {
    width * height
}

fn area_second_example(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_third_examle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn dbg_example() {
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
