use serde::Serialize;

use crate::entities::tank_pressure::structure::TankPressure;

/// Represents a waypoint.
///
/// https://www.streit.cc/extern/uddf_v321/en/waypoint.html
#[derive(Debug, Serialize)]
pub struct Waypoint {
    /// In meters.
    pub depth: f64,
    /// Passed time since the beginning of the dive, in seconds.
    #[serde(rename = "divetime")]
    pub dive_time: u64,
    // In pascals.
    #[serde(rename = "tankpressure", skip_serializing_if = "Option::is_none")]
    pub tank_pressure: Option<TankPressure>,
    /// In Kelvin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}
