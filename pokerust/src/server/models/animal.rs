use tide::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Animal {
    pub name: String,
    pub legs: u8,
}
