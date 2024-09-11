use serde::Serialize;

use crate::entities::{
    information_after_dive::structure::InformationAfterDive,
    information_before_dive::structure::InformationBeforeDive, samples::structure::Samples,
    tank_data::structure::TankData,
};

/// Represents a dive.
///
/// https://www.streit.cc/extern/uddf_v321/en/dive.html
#[derive(Debug, Serialize)]
pub struct Dive {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "informationbeforedive")]
    pub information_before_dive: InformationBeforeDive,
    #[serde(rename = "tankdata", skip_serializing_if = "Option::is_none")]
    pub tank_data: Option<TankData>,
    pub samples: Vec<Samples>,
    #[serde(rename = "informationafterdive")]
    pub information_after_dive: InformationAfterDive,
}
