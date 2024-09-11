use serde::Serialize;

use crate::entities::repetition_group::structure::RepetitionGroup;

/// Represents a profile data.
///
/// https://www.streit.cc/extern/uddf_v321/en/profiledata.html
#[derive(Debug, Serialize)]
pub struct ProfileData {
    #[serde(rename = "repetitiongroup")]
    pub repetition_groups: Vec<RepetitionGroup>,
}
