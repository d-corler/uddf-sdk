use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::link::structure::Link;

/// Represents information before dive.
///
/// https://www.streit.cc/extern/uddf_v321/en/informationbeforedive.html
#[derive(Debug, Serialize)]
pub struct InformationBeforeDive {
    pub link: Vec<Link>,
    /// Total number of dives since beginning of recording by the diver.
    #[serde(rename = "divenumber")]
    pub dive_number: u64,
    // In pascals.
    #[serde(rename = "datetime")]
    pub date_time: DateTime<Utc>,
}
