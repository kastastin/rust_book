use crate::front_of_house::hosting;

#[allow(dead_code)]
mod front_of_house;

#[allow(dead_code)]
mod back_of_house;

#[allow(dead_code)]
mod customer;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = "Wheat".to_string();

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("order1 = {:?}, order2 = {:?}", order1, order2);
}

fn deliver_order() {}
