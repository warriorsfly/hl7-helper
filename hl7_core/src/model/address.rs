use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Address {
    #[serde(rename = "use", default)]
    pub xuse: String,
    /// 值可以为空
    #[serde(rename = "houseNumber", default)]
    pub house_number: Option<String>,
    #[serde(rename = "streetName", default)]
    pub street_name: Option<String>,
    #[serde(rename = "township", default)]
    pub town_ship: Option<String>,
    pub county: Option<String>,
    #[serde(rename = "houseNumber", default)]
    pub city: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "postalCode", default)]
    pub postal_code: Option<String>,
}

impl Default for Address {}
