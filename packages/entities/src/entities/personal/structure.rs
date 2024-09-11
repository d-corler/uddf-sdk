use serde::Serialize;

use crate::entities::{first_name::structure::FirstName, last_name::structure::LastName};

/// Represents a personal.
///
/// https://www.streit.cc/extern/uddf_v321/en/personal.html
#[derive(Debug, Serialize)]
pub struct Personal {
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<FirstName>,
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<LastName>,
}
