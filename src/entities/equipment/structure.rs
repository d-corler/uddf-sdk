use serde::Serialize;

use crate::entities::{dive_computer::structure::DiveComputer, light::structure::Light};

/// Enumeration of all possible equipments.
#[derive(Debug, Serialize)]
pub enum Equipment_ {
    #[serde(rename = "divecomputer")]
    DiveComputer(DiveComputer),
    #[serde(rename = "light")]
    Light(Light),
}

/// Represents a set of equipment.
///
/// https://www.streit.cc/extern/uddf_v321/en/equipment.html
#[derive(Debug, Serialize)]
pub struct Equipment {
    #[serde(rename = "$value")]
    pub equipments: Vec<Equipment_>,
}
