use serde::Serialize;

/// Represents information after dive.
///
/// https://www.streit.cc/extern/uddf_v321/en/informationafterdive.html
#[derive(Debug, Serialize)]
pub struct InformationAfterDive {
    /// In meters.
    #[serde(rename = "greatestdepth", skip_serializing_if = "Option::is_none")]
    pub greatest_depth: Option<f64>,
    /// In meters.
    #[serde(rename = "averagedepth", skip_serializing_if = "Option::is_none")]
    pub average_depth: Option<f64>,
    // In seconds.
    #[serde(rename = "diveduration", skip_serializing_if = "Option::is_none")]
    pub dive_duration: Option<f64>,
}
