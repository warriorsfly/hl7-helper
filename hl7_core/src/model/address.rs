use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Address {
    #[serde(rename = "use", default)]
    pub xuse: String,
    /// 值可以为空
    #[serde(rename = "houseNumber", default)]
    pub house_number: String,
    #[serde(rename = "streetName", default)]
    pub street_name: String,
    #[serde(rename = "township", default)]
    pub town_ship: String,
    pub county: String,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            xuse: "H".into(),
            house_number: "".into(),
            street_name: "".into(),
            town_ship: "".into(),
            county: "".into(),
            city: "".into(),
            state: "".into(),
            postal_code: "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    /// 简单判断 default addr能否被读取
    #[test]
    fn test_default_addr() {
        let addr = Address::default();
        let read_addr: Address =
            from_str("<addr use=\"H\"><houseNumber/><streetName/><township/><county/><city/><state/><postalCode/></addr>")
                .expect("错误的xml格式");
        assert_eq!(addr, read_addr);
    }
}
