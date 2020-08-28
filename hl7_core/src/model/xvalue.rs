use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct XValue {
    pub value: String,
}

impl Default for XValue {
    fn default() -> Self {
        Self { value: "".into() }
    }
}
