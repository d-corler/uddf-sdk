use serde::Serialize;

use crate::entities::dive::structure::Dive;

/// Represents a repetition group.
///
/// https://www.streit.cc/extern/uddf_v321/en/repetitiongroup.html
#[derive(Debug, Serialize)]
pub struct RepetitionGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "dive")]
    pub dives: Vec<Dive>,
}
