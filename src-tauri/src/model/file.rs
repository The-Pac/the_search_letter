use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct File {
    pub size: u64,
    pub percents: String,
    pub name: String,
}

impl File {
    pub fn new(size: u64, name: String, percents: String) -> Self {
        Self { size, percents, name }
    }
}