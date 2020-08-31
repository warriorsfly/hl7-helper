use serde::Deserialize;

/// 地域代码
#[derive(Debug, Deserialize, PartialEq)]
pub struct RealmCode {
    pub code: String,
}

impl Default for RealmCode {
    fn default() -> Self {
        Self {
            /// 默认地域:中国
            code: "CN".to_string(),
        }
    }
}
