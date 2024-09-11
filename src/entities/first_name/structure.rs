use serde::Serialize;

/// Represents a first name.
///
/// https://www.streit.cc/extern/uddf_v321/en/firstname.html
#[derive(Debug, Serialize)]
pub struct FirstName {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
