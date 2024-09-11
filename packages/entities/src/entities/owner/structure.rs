use serde::Serialize;

use crate::entities::{equipment::structure::Equipment, personal::structure::Personal};

/// Represents an owner.
///
/// https://www.streit.cc/extern/uddf_v321/en/owner.html
#[derive(Debug, Serialize)]
pub struct Owner {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<Personal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Equipment>,
}
