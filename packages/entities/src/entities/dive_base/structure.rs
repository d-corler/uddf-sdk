use serde::Serialize;

/// Represents a dive base.
///
/// https://www.streit.cc/extern/uddf_v321/en/divebase.html
#[derive(Debug, Serialize)]
pub struct DiveBase {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
