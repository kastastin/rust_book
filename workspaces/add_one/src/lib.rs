pub fn add_one(x: u8) -> u8 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(9), 10);
    }
}
