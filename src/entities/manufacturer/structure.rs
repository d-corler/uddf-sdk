use serde::Serialize;

use crate::entities::contact::structure::Contact;

/// Represents a manufacturer.
///
/// https://www.streit.cc/extern/uddf_v321/en/manufacturer.html
#[derive(Debug, Serialize)]
pub struct Manufacturer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}
