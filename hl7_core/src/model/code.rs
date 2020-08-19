use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Code {
    /// 编码
    pub code: String,
    /// 显示名称
    #[serde(rename = "displayName", default)]
    pub display_name: String,
    ///代码体系
    // todo!("体系化")
    #[serde(rename = "codeSystem", default)]
    pub code_system: String,

    ///代码体系
    // todo!("体系化")
    #[serde(rename = "codeSystemName", default)]
    pub code_system_name: String,
}
