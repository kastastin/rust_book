pub fn generic_type_examples() {
    first_example();
    second_example();
    third_example();
}

fn first_example() {
    let number_list = vec![15, 5, 10, 50, 25];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result1 = largest(&number_list);
    let result2 = largest(&char_list);

    println!(
        "------------\n\
        Generics - 1:\n\
        result1 = {result1}\n\
        result2 = {result2}\n\
        ------------"
    );

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}

fn second_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let integer = Point { x: 5, y: 5 };
    let float = Point { x: 0.5, y: 0.5 };

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Position<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Position { x: 10, y: 10 };
    let both_float = Position { x: 1.5, y: 1.5 };
    let integer_and_float = Position { x: 5, y: 2.5 };

    println!(
        "Generics - 2:\n\
        integer.x = {}\n\
        integer = {:?}\n\
        float = {:?}\n\
        both_integer = {:?}\n\
        both_float = {:?}\n\
        integer_and_float = {:?}\n\
        ------------",
        integer.x(),
        integer,
        float,
        both_integer,
        both_float,
        integer_and_float
    );
}

fn third_example() {
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!(
        "Generics - 3:\n\
        p3.x = {}\n\
        p3.y = {}\n\
        ------------",
        p3.x, p3.y
    );
}
