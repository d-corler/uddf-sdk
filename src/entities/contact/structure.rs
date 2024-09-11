use serde::Serialize;

/// Represents a contact.
///
/// https://www.streit.cc/extern/uddf_v321/en/contact.html
#[derive(Debug, Serialize)]
pub struct Contact {
    /// Homepage URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
}
