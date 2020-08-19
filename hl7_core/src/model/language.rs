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
