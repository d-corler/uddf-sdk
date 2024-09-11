use serde::Serialize;

use crate::entities::owner::structure::Owner;

/// Represents a diver.
///
/// https://www.streit.cc/extern/uddf_v321/en/diver.html
#[derive(Debug, Serialize)]
pub struct Diver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}
