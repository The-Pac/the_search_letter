use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Letter {
    pub number: isize,
    pub char: char,
}

impl Letter {
    pub fn new(number: isize, char: char) -> Self {
        Self { number, char }
    }
}