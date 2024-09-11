use serde::Serialize;

use crate::entities::{dive_base::structure::DiveBase, site::structure::Site};

/// Represents a dive site.
///
/// https://www.streit.cc/extern/uddf_v321/en/divesite.html
#[derive(Debug, Serialize)]
pub struct DiveSite {
    #[serde(rename = "divebase")]
    pub dive_base: DiveBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}
