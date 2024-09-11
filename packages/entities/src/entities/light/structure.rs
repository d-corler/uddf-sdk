use serde::Serialize;

/// Represents a light.
///
/// https://www.streit.cc/extern/uddf_v321/en/light.html
#[derive(Debug, Serialize)]
pub struct Light {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
