use serde::Serialize;

use crate::entities::geography::structure::Geography;

/// Represents a site.
///
/// https://www.streit.cc/extern/uddf_v321/en/site.html
#[derive(Debug, Serialize)]
pub struct Site {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geography: Option<Geography>,
}
