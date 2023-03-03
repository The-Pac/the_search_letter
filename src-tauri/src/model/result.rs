use serde::Serialize;

use crate::model::file::File;
use crate::model::letter::Letter;

#[derive(Serialize)]
pub struct Result {
    pub chars_result: Vec<Letter>,
    pub files_result: Vec<File>,
    pub duration: u64,
}

impl Result {
    pub fn new(chars_result: Vec<Letter>, files_result: Vec<File>, duration: u64) -> Self {
        Self { chars_result, files_result, duration }
    }
}
