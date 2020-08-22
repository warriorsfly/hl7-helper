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

    /// 简单判断 default id能否被读取
    #[test]
    fn test_default_id() {
        let id = Id::default();
        let read_id: Id =
            from_str("<id root=\"2.16.840.1.113883.1.3\" extension=\"POCD_MT000040\"/>")
                .expect("错误的xml格式");
        assert_eq!(id, read_id);
    }

    /// 判定extension==None的时候xml的格式
    #[test]
    fn test_none_extension_id() {
        let id = Id {
            root: "2.16.840.1.113883.1.3".into(),
            extension: None,
        };

        let read_id: Id = from_str("<id root=\"2.16.840.1.113883.1.3\"/>").expect("错误的xml格式");
        assert_eq!(id, read_id);
    }
}
