use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct XTime {
    #[serde(rename = "xsi:type", default)]
    pub xtype: String,
    /// 值可以为空
    pub value: String,
}
