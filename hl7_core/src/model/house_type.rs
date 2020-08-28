use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct HouseType {
    #[serde(rename = "xsi:type", default)]
    pub xtype: String,
    pub value: String,
}

impl Default for HouseType {
    fn default() -> Self {
        Self {
            xtype: "BL".into(),
            value: "true".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_default_house_type() {
        let house = HouseType::default();
        let read_house: HouseType =
            from_str("<houseType xsi:type=\"BL\" value= \"true\"/>").expect("错误的xml格式");
        assert_eq!(house, read_house);
    }
}
