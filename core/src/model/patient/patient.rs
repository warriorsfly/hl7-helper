use crate::model::{Code, EducationLevel, HouseHold, Id, Occupation, Organization, XValue};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Patient {
    #[serde(rename = "classCode", default)]
    pub class_code: String,

    #[serde(rename = "determinerCode", default)]
    pub determiner_code: String,

    pub id: Id,
    pub name: String,
    /// 性别代码
    #[serde(rename = "administrativeGenderCode", default)]
    pub administrative_gender_code: Code,
    /// 出生日期
    #[serde(rename = "birthTime", default)]
    pub birth_time: XValue,

    /// 婚姻状态
    #[serde(rename = "maritalStatusCode", default)]
    pub marital_status_code: Code,

    /// 民族
    #[serde(rename = "ethnicGroupCode", default)]
    pub ethnic_group_code: Code,
    /// 工作单位
    #[serde(rename = "employerOrganization", default)]
    pub employer_organization: Organization,
    /// 是否原住民
    #[serde(rename = "household", default)]
    pub house_hold: HouseHold,
    /// 教育程度
    #[serde(rename = "educationLevel", default)]
    pub education_level: EducationLevel,
    /// 教育程度
    pub occupation: Occupation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HouseType;
    use quick_xml::de::from_str;
    #[test]
    fn test_patient_struct() {
        let read_patient: Patient =
            from_str("<patient classCode=\"PSN\" determinerCode=\"INSTANCE\">
        <id root=\"2.16.156.10011.1.3\" extension=\"42010619630312203X\"/>
        <name>曾祥明</name>
        <administrativeGenderCode code=\"1\" displayName=\"男性\" codeSystem=\"2.16.156.10011.2.3.3.4\" codeSystemName=\"生理性别代码表(GB/T 2261.1)\"/>
        <birthTime value=\"19630312\"/>
        <maritalStatusCode code=\"20\" displayName=\"已婚\" codeSystem=\"2.16.156.10011.2.3.3.5\" codeSystemName=\"婚姻状况代码表(GB/T 2261.2)\"/>
        <ethnicGroupCode code=\"01\" displayName=\"汉族\" codeSystem=\"2.16.156.10011.2.3.3.3\" codeSystemName=\"民族类别代码表(GB 3304)\"/>
        <employerOrganization>
          <name>/</name>
        </employerOrganization>
        <household>
          <houseType xsi:type=\"BL\" value=\"true\"/>
        </household>
        <educationLevel>
          <educationLevelCode code=\"60\" displayName=\"普通高级中学教育\" codeSystem=\"2.16.156.10011.2.3.3.6\" codeSystemName=\"学历代码表(GB/T 4658)\"/>
        </educationLevel>
        <occupation>
          <occupationCode code=\"Y\" displayName=\"不便分类的其他从业人员\" codeSystem=\"2.16.156.10011.2.3.3.7\" codeSystemName=\"职业类别代码表(GB/T 6565)\"/>
        </occupation>
      </patient>")
                .expect("错误的xml格式");

        let new_patient = Patient {
            class_code: "PSN".into(),
            determiner_code: "INSTANCE".into(),
            id: Id {
                root: "2.16.156.10011.1.3".into(),
                extension: Some("42010619630312203X".into()),
            },
            name: "曾祥明".into(),
            administrative_gender_code: Code {
                code: "1".into(),
                display_name: Some("男性".into()),
                code_system: "2.16.156.10011.2.3.3.4".into(),
                code_system_name: "生理性别代码表(GB/T 2261.1)".into(),
            },
            birth_time: XValue {
                value: "19630312".into(),
            },
            marital_status_code: Code {
                code: "20".into(),
                display_name: Some("已婚".into()),
                code_system: "2.16.156.10011.2.3.3.5".into(),
                code_system_name: "婚姻状况代码表(GB/T 2261.2)".into(),
            },
            ethnic_group_code: Code {
                code: "01".into(),
                display_name: Some("汉族".into()),
                code_system: "2.16.156.10011.2.3.3.3".into(),
                code_system_name: "民族类别代码表(GB 3304)".into(),
            },
            employer_organization: Organization::default(),
            house_hold: HouseHold {
                house_type: HouseType::default(),
            },
            education_level: EducationLevel {
                code: Code {
                    code: "60".into(),
                    display_name: Some("普通高级中学教育".into()),
                    code_system: "2.16.156.10011.2.3.3.6".into(),
                    code_system_name: "学历代码表(GB/T 4658)".into(),
                },
            },
            occupation: Occupation::default(),
        };

        assert_eq!(read_patient, new_patient);
    }
}
