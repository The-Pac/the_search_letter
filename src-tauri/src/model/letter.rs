use serde::Serialize;

#[derive(Serialize)]
pub struct Letter {
    pub byte: u8,
    pub number: isize,
    pub char: char,
}

impl Letter {
    pub fn new(byte: u8, number: isize, char: char) -> Self {
        Self { byte, number, char }
    }
}