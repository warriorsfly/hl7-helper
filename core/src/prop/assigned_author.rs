use crate::model::{Id, Organization, Person, XTime};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Validate)]
pub struct AssignedAuthor {
    #[serde(rename = "classCode", default)]
    pub class_code: String,

    pub id: Id,
    #[serde(rename = "assignedPerson", default)]
    pub assigned_person: Person,
    #[serde(rename = "representedOrganization")]
    pub represented_organization: Organization,
}
