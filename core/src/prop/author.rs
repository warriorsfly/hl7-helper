use crate::model::XTime;
use serde::Deserialize;
use validator::Validate;

use super::AssignedAuthor;

#[derive(Debug, Deserialize, PartialEq, Validate)]
pub struct Author {
    // #[validate(must_match = "AUT")]
    #[serde(rename = "typeCode", default)]
    pub type_code: String,
    // #[validate(must_match = "OP")]
    #[serde(rename = "contextControlCode", default)]
    pub context_control_code: String,

    pub time: XTime,

    pub assigned_author: AssignedAuthor,
}
