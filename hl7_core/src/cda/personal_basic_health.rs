/// 第1部分:个人基本健康信息登记
use model::{Code, Id, Language, RealmCode, XTime};
use serde::Deserialize;

/// 地域代码
#[derive(Debug, Deserialize, PartialEq)]
pub struct PersonalBasicHealth {
    /// 区域代码
    #[serde(rename = "realmCode", default)]
    pub realm_code: RealmCode,
    /// 文档注册模型
    #[serde(rename = "typeId", default)]
    pub type_id: Id,
    /// 文档流水号
    pub id: Id,
    /// 文档类型
    pub code: Code,
    /// 文档标题,此处为:个人基本健康信息登记
    #[serde(rename = "$value")]
    pub title: String,
    /// 文档创建时间
    #[serde(rename = "effectiveTime")]
    pub effective_time: XTime,
    /// 文档密级代码,缺省为:2.16.840.1.113883.5.25
    #[serde(rename = "confidentialityCode")]
    pub confidentiality_code: Code,
    /// 语言 默认:zh-CN
    #[serde(rename = "languageCode")]
    pub language_code: Language,
}
