use serde::Serialize;

/// Represents the tank pressure.
///
/// https://www.streit.cc/extern/uddf_v321/en/tankpressure.html
#[derive(Debug, Serialize)]
pub struct TankPressure {
    #[serde(rename = "$text")]
    pub value: f64,
}
