#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: toast.to_string(),
            seasonal_fruit: "peaches".to_string(),
        }
    }
}

fn fix_incorrect_order() {
  cook_order();
  super::deliver_order();
}

fn cook_order() {}