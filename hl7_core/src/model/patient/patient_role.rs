use crate::model::{Address, Id};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct PatientRole {
    /// 值可以为空
    #[serde(rename = "classCode", default)]
    pub class_code: String,
    pub id: Id,
    pub addr: Address,
    /// 文档标题,此处为:个人基本健康信息登记
    #[serde(rename = "$value")]
    pub telecom: String,
}
