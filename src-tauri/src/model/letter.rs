use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Letter {
    pub number: u128,
    pub percents: String,
    pub char: char,
}

impl Letter {
    pub fn new(number: u128, char: char, percents: String) -> Self {
        Self { number, percents, char }
    }
}