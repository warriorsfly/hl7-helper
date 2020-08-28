use crate::model::Code;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EducationLevel {
    #[serde(rename = "educationLevelCode", default)]
    pub code: Code,
}

impl Default for EducationLevel {
    fn default() -> Self {
        Self {
            code: Code::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_default_education_code() {
        let code = Code {
            code: "60".into(),
            display_name: Some("普通高级中学教育".into()),
            code_system: "2.16.156.10011.2.3.3.6".into(),
            code_system_name: "学历代码表(GB/T 4658)".into(),
        };
        let read_code: EducationLevel =
            from_str("<educationLevel><educationLevelCode code=\"60\" displayName=\"普通高级中学教育\" codeSystem=\"2.16.156.10011.2.3.3.6\" codeSystemName=\"学历代码表(GB/T 4658)\"/></educationLevel>")
                .expect("错误的xml格式");
        assert_eq!(code, read_code.code);
    }
}
