use serde::Serialize;

use crate::model::letter::Letter;

#[derive(Serialize)]
pub struct Result {
    pub chars_result: Vec<Letter>,
    pub duration: u64,
}

impl Result {
    pub fn new(chars_result: Vec<Letter>, duration: u64) -> Self {
        Self { chars_result, duration }
    }
}
