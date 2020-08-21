use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Id {
    pub root: String,
    /// 值可以为空
    pub extension: Option<String>,
}

impl Default for Id {
    fn default() -> Self {
        Self {
            root: "2.16.840.1.113883.1.3".into(),
            extension: Some("POCD_MT000040".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn default_id() {
        let id = Id::default();
        let read_id: Id =
            from_str("<id root=\"2.16.840.1.113883.1.3\" extension=\"POCD_MT000040\"/>")
                .expect("错误的xml格式");
        assert_eq!(id, read_id);
    }
}
