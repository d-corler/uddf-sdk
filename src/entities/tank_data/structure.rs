use serde::Serialize;

use crate::entities::{link::structure::Link, tank_pressure::structure::TankPressure};

/// Represents a tank data.
///
/// https://www.streit.cc/extern/uddf_v321/en/tankdata.html
#[derive(Debug, Serialize)]
pub struct TankData {
    pub link: Vec<Link>,
    /// In cubic-meters [m^3].
    #[serde(rename = "tankvolume", skip_serializing_if = "Option::is_none")]
    pub tank_volume: Option<f64>,
    // In pascals.
    #[serde(rename = "tankpressurebegin", skip_serializing_if = "Option::is_none")]
    pub tank_pressure_begin: Option<TankPressure>,
    // In pascals.
    #[serde(rename = "tankpressureend", skip_serializing_if = "Option::is_none")]
    pub tank_pressure_end: Option<TankPressure>,
}
