use crate::model::HouseType;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct HouseHold {
    #[serde(rename = "houseType", default)]
    pub house_type: HouseType,
}

impl Default for HouseHold {
    fn default() -> Self {
        Self {
            house_type: HouseType::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_default_house_hold() {
        let house = HouseHold::default();
        let read_house: HouseHold =
            from_str("<household><houseType xsi:type=\"BL\" value= \"true\"/></household>")
                .expect("错误的xml格式");
        assert_eq!(house, read_house);
    }
}
