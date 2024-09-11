use serde::Serialize;

/// Represents a last name.
///
/// https://www.streit.cc/extern/uddf_v321/en/lastname.html
#[derive(Debug, Serialize)]
pub struct LastName {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
