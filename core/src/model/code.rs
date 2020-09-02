use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Code {
    /// 编码
    pub code: String,
    /// 显示名称
    #[serde(rename = "displayName", default)]
    pub display_name: Option<String>,
    ///代码体系
    // todo!("体系化")
    #[serde(rename = "codeSystem", default)]
    pub code_system: String,

    ///代码体系
    // todo!("体系化")
    #[serde(rename = "codeSystemName", default)]
    pub code_system_name: String,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            code: "HSDA00.01".into(),
            display_name: None,
            code_system: "2.16.156.10011.2.4".into(),
            code_system_name: "卫生信息共享文档规范编码体系".into(),
        }
    }
}

impl Code {
    fn default_personal_basic_health_code() -> Self {
        Self {
            code: "HSDA00.01".into(),
            display_name: None,
            code_system: "2.16.156.10011.2.4".into(),
            code_system_name: "卫生信息共享文档规范编码体系".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_default_personal_basic_health_code() {
        let code = Code::default_personal_basic_health_code();
        let read_code: Code =
            from_str("<code code=\"HSDA00.01\" codeSystem=\"2.16.156.10011.2.4\" codeSystemName=\"卫生信息共享文档规范编码体系\"/>")
                .expect("错误的xml格式");
        assert_eq!(code, read_code);
    }
}
