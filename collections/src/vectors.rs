pub fn vector_examples() {
    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
}

fn first_example() {
    let v1: Vec<i32> = vec![10, 20, 30];
    println!("Vectors - 1: v1 = {:?}", v1);

    let mut v2: Vec<u8> = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    println!("Vectors - 1: v2 = {:?}", v2);

    let v2_third1 = v2[2];
    let v2_third2 = &v2[2];
    let v2_third3 = v2.get(2);
    println!("Vectors - 1: v2_third1 = {v2_third1}");
    println!("Vectors - 1: v2_third2 = {v2_third2}");
    println!("Vectors - 1: v2_third2 = {:?}", v2_third3);
}

fn second_example() {
    let v: Vec<u8> = vec![5, 10, 15];

    // let v_tenth1 = v[10]; // panic
    // println!("Vectors - 2: v_tenth1 = {v_tenth1}");

    let v_tenth2 = v.get(10);
    println!("Vectors - 2: v_tenth2 = {:?}", v_tenth2); // None
}

#[allow(unused_mut)]
fn third_example() {
    let mut v: Vec<u8> = vec![3, 6, 9];

    let v_first = &v[0];

    // you cannot have mutable and non-mutable links in the same scope
    // v.push(12); // cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("Vectors - 3: v_first = {v_first}");
}

fn fourth_example() {
    let v1: Vec<u8> = vec![50, 100, 150];

    for i in &v1 {
        println!("Vectors - 4: v1[?] = {i}");
    }

    let mut v2 = vec![3, 4, 5];

    for i in &mut v2 {
        *i *= 10;
        println!("Vectors - 4: v2[?] = {i}");
    }
}

fn fifth_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(55),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text("Black".to_string()),
    ];

    println!("Vectors - 5: row = {:?}", row);
}
