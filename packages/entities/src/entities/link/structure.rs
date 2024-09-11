use serde::Serialize;

/// Represents the link.
///
/// https://www.streit.cc/extern/uddf_v321/en/link.html
#[derive(Debug, Serialize)]
pub struct Link {
    #[serde(rename = "@ref")]
    pub reference: String,
}
