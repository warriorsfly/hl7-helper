use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Person {
    pub name: String,
}

impl Default for Person {
    fn default() -> Self {
        Self { name: "".into() }
    }
}
