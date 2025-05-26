struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    White,
    Black,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_white = 0;
        let mut num_black = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::White => num_white += 1,
                ShirtColor::Black => num_black += 1,
            }
        }

        if num_white > num_black {
            ShirtColor::White
        } else {
            ShirtColor::Black
        }
    }
}

pub fn example() {
    let store = Inventory {
        shirts: vec![ShirtColor::White, ShirtColor::Black, ShirtColor::White],
    };

    let user_pref1 = Some(ShirtColor::Black);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
