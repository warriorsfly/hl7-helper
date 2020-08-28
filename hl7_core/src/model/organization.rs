use super::Id;
use serde::Deserialize;

/// 机构信息
#[derive(Debug, Deserialize, PartialEq)]
pub struct Organization {
    /// 地址
    pub addr: Option<String>,
    pub id: Option<Id>,
    /// 名称
    pub name: String,
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            addr: None,
            id: None,
            name: "/".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    #[test]
    fn test_employee_organization() {
        let org = Organization::default();
        let read_org: Organization =
            from_str("<employerOrganization><name>/</name></employerOrganization>")
                .expect("错误的xml格式");
        assert_eq!(org, read_org);
    }
}
