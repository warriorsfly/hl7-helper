use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Address {
    pub root: String,
    /// 值可以为空
    pub extension: Option<String>,
}

impl Id {
    /// 默认typeId
    pub fn default_type_id() -> Self {
        Self {
            root: "2.16.840.1.113883.1.3".into(),
            extension: Some("POCD_MT000040".into()),
        }
    }
}
