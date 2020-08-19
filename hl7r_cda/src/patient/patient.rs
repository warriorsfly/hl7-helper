use super::Id;
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq)]
pub struct Patient {
    pub id: Id,
    /// 姓名
    pub name: String,
    /// 生日
    #[serde(rename = "birthTime", default)]
    pub birth_time: String,
}
