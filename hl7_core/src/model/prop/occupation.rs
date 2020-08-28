use crate::model::Code;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Occupation {
    #[serde(rename = "occupationCode", default)]
    pub code: Code,
}

impl Default for Occupation {
    fn default() -> Self {
        Self {
            code: Code {
                code: "Y".into(),
                display_name: Some("不便分类的其他从业人员".into()),
                code_system: "2.16.156.10011.2.3.3.7".into(),
                code_system_name: "职业类别代码表(GB/T 6565)".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_default_occupation_code() {
        let item = Occupation::default();
        let read_item: Occupation =
            from_str("<occupation><occupationCode code=\"Y\" displayName=\"不便分类的其他从业人员\" codeSystem=\"2.16.156.10011.2.3.3.7\" codeSystemName=\"职业类别代码表(GB/T 6565)\"/></occupation>")
                .expect("错误的xml格式");
        assert_eq!(item, read_item);
    }
}
