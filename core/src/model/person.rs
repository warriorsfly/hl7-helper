use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Person {
    pub name: String,
}
