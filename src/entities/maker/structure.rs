use serde::Serialize;

use crate::entities::manufacturer::structure::Manufacturer;

/// Represents a maker.
///
/// https://www.streit.cc/extern/uddf_v321/en/maker.html
#[derive(Debug, Serialize)]
pub struct Maker {
    #[serde(rename = "manufacturer")]
    pub manufacturers: Vec<Manufacturer>,
}
