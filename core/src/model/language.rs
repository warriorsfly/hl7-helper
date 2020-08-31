use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Language {
    /// 编码
    pub code: String,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            code: "zh-CN".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    /// 简单判断 default 能否被读取
    #[test]
    fn test_default_language() {
        let language = Language::default();
        let readed_language: Language =
            from_str("<languageCode code=\"zh-CN\"/>").expect("错误的xml格式");
        assert_eq!(language, readed_language);
    }
}
